use ash::vk;

use winit::window::Window;

use std::error::Error;

use crate::scop::vulkan::device::Device;

use crate::scop::vulkan::renderpass::RenderPass;

use crate::scop::vulkan::surface::Surface;

pub struct Swapchain {
    pub raw: vk::SwapchainKHR,
    pub loader: ash::khr::swapchain::Device,
    pub images: Vec<vk::Image>,
    pub images_view: Vec<vk::ImageView>,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub extent: vk::Extent2D,
}

impl Swapchain {
    pub fn new(
        window: &Window,
        instance: &ash::Instance,
        surface: &Surface,
        device: &Device,
    ) -> Result<Self, Box<dyn Error>> {
        let queue_family = [device.graphic_family_index];

        let capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(device.physical, surface.raw)?
        };

        let present_modes = unsafe {
            surface
                .loader
                .get_physical_device_surface_present_modes(device.physical, surface.raw)?
        };

        let present_mode = present_modes
            .into_iter()
            .max_by_key(|mode| match *mode {
                vk::PresentModeKHR::MAILBOX => 3,
                vk::PresentModeKHR::IMMEDIATE => 2,
                vk::PresentModeKHR::FIFO => 1,
                vk::PresentModeKHR::FIFO_RELAXED => 0,
                _ => 0,
            })
            .expect("no valid vulkan present mode ");

        let formats = unsafe {
            surface
                .loader
                .get_physical_device_surface_formats(device.physical, surface.raw)?
        };

        let (format, color_space) = {
            formats
                .iter()
                .filter(|surface_format| {
                    surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
                })
                .map(|format| (format.format, format.color_space))
                .max_by_key(|(format, _)| match *format {
                    vk::Format::B8G8R8A8_SRGB => 2,
                    vk::Format::R8G8B8A8_SRGB => 2,
                    vk::Format::B8G8R8A8_UNORM => 1,
                    vk::Format::R8G8B8A8_UNORM => 1,
                    _ => 0,
                })
                .expect("no supported surface format found on this device")
        };

        let image_count = if capabilities.max_image_count == 0 {
            3.max(capabilities.min_image_count)
        } else {
            3.max(capabilities.min_image_count)
                .min(capabilities.max_image_count)
        };

        let swapchain_extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            vk::Extent2D {
                // clamp 100.clamp(50, 200) clamp check si la value est bien entre les deux value donner
                width: window.inner_size().width.clamp(
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: window.inner_size().height.clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.raw)
            .min_image_count(image_count)
            .image_format(format)
            .image_color_space(color_space)
            .image_extent(swapchain_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_family)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode);

        let swapchain_loader = ash::khr::swapchain::Device::new(instance, &device.logical);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };

        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };

        let mut swapchain_images_views = Vec::with_capacity(swapchain_images.len());

        for image in swapchain_images.iter() {
            let subresource_range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let images_view_create_info = vk::ImageViewCreateInfo::default()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .subresource_range(subresource_range);
            let images_view = unsafe {
                device
                    .logical
                    .create_image_view(&images_view_create_info, None)
            }?;
            swapchain_images_views.push(images_view);
        }

        Ok(Self {
            raw: swapchain,
            loader: swapchain_loader,
            images: swapchain_images,
            images_view: swapchain_images_views,
            format,
            color_space,
            framebuffers: Vec::new(),
            extent: swapchain_extent,
        })
    }

    pub fn create_framebuffers(
        &mut self,
        device: &Device,
        renderpass: &RenderPass,
    ) -> Result<(), Box<dyn Error>> {
        for image in self.images_view.iter() {
            let image_view = [*image];
            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(renderpass.raw)
                .attachments(&image_view)
                .width(self.extent.width)
                .height(self.extent.height)
                .layers(1);

            let framebuffer =
                unsafe { device.logical.create_framebuffer(&framebuffer_info, None)? };

            self.framebuffers.push(framebuffer);
        }
        Ok(())
    }

    pub fn clean(&self, device: &Device) {
        unsafe {
            for framebuffer in self.framebuffers.iter() {
                device.logical.destroy_framebuffer(*framebuffer, None);
            }
            for image in self.images_view.iter() {
                device.logical.destroy_image_view(*image, None);
            }

            self.loader.destroy_swapchain(self.raw, None)
        };
    }
}
