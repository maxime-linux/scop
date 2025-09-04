use ash::vk;

use winit::window::Window;

use std::error::Error;

use crate::scop::vulkan_setup::device::Device;

use crate::scop::vulkan_setup::surface::Surface;

pub struct Swapchain {
    pub raw: vk::SwapchainKHR,
    pub loader: ash::khr::swapchain::Device,
    pub images_view: Vec<vk::ImageView>,
}

impl Swapchain {
    pub fn new(
        window: &Window,
        instance: &ash::Instance,
        surface: &Surface,
        device: &Device,
    ) -> Result<Self, Box<dyn Error>> {
        let surface_capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(device.physical, surface.raw)?
        };

        let surface_present_modes = unsafe {
            surface
                .loader
                .get_physical_device_surface_present_modes(device.physical, surface.raw)?
        };

        let surface_formats = unsafe {
            surface
                .loader
                .get_physical_device_surface_formats(device.physical, surface.raw)?
        };

        let queue_family = [device.graphic_family_index];

        let image_count = if surface_capabilities.max_image_count == 0 {
            3.max(surface_capabilities.min_image_count)
        } else {
            3.max(surface_capabilities.min_image_count)
                .min(surface_capabilities.max_image_count)
        };

        let swapchain_extent = if surface_capabilities.current_extent.width != u32::MAX {
            surface_capabilities.current_extent
        } else {
            vk::Extent2D {
                // clamp 100.clamp(50, 200) clamp check si la value est bien entre les deux value donner
                width: window.inner_size().width.clamp(
                    surface_capabilities.min_image_extent.width,
                    surface_capabilities.max_image_extent.width,
                ),
                height: window.inner_size().height.clamp(
                    surface_capabilities.min_image_extent.height,
                    surface_capabilities.max_image_extent.height,
                ),
            }
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.raw)
            .min_image_count(image_count)
            .image_format(surface_formats.first().unwrap().format)
            .image_color_space(surface_formats.first().unwrap().color_space)
            .image_extent(swapchain_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_family)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::MAILBOX);

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
                .format(surface_formats.first().unwrap().format)
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
            images_view: swapchain_images_views,
        })
    }
}
