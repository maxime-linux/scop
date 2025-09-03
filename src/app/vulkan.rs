use ash::{ext::debug_utils, khr::surface, vk, Entry, Instance};

use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use winit::window::Window;

use std::ffi::{c_char, c_void, CStr};

use std::error::Error;

struct DebugUtils {
    instance: debug_utils::Instance,
    message: vk::DebugUtilsMessengerEXT,
}

struct Surface {
    raw: vk::SurfaceKHR,
    loader: surface::Instance,
}

struct Device {
    graphic_queue: vk::Queue,
    transfer_queue: vk::Queue,
    graphic_family_index: u32,
    transfer_family_index: u32,
    logical: ash::Device,
    physical: vk::PhysicalDevice,
}

struct Swapchain {
    raw: vk::SwapchainKHR,
    loader: ash::khr::swapchain::Device,
    images_view: Vec<vk::ImageView>,
}

pub struct VulkanCore {
    entry: Entry,
    instance: Instance,
    debug: DebugUtils,
    surface: Surface,
    device: Device,
    swapchain: Swapchain,
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    msg_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let message = unsafe { CStr::from_ptr((*callback_data).p_message) };
    let severity = format!("{:?}", msg_severity).to_lowercase();
    let ty = format!("{:?}", msg_type).to_lowercase();
    eprintln!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}

impl VulkanCore {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let (entry, instance, debug) = Self::create_vulkan_base(window)?;
        let surface = Self::create_vulkan_surface(window, &entry, &instance)?;
        let device = Self::create_vulkan_device(&instance, &surface)?;
        let swapchain = Self::create_vulkan_swapchain(window, &instance, &surface, &device)?;
        Ok(Self {
            entry,
            instance,
            debug,
            surface,
            device,
            swapchain,
        })
    }

    fn create_vulkan_base(
        window: &Window,
    ) -> Result<(Entry, Instance, DebugUtils), Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };

        let app_info: vk::ApplicationInfo = vk::ApplicationInfo::default()
            // .application_name(c"scop")
            .application_version(vk::make_api_version(0, 1, 0, 0))
            // .engine_name(c"scop_engine")
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_3);

        let layer_name: Vec<*const c_char> = vec![c"VK_LAYER_KHRONOS_validation".as_ptr()];

        let mut instance_extensions: Vec<*const c_char> =
            ash_window::enumerate_required_extensions(window.display_handle()?.as_raw())?.to_vec();
        instance_extensions.push(ash::ext::debug_utils::NAME.as_ptr());

        let mut debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            )
            .pfn_user_callback(Some(vulkan_debug_utils_callback));

        let instance_create_info = vk::InstanceCreateInfo::default()
            .push_next(&mut debug_create_info)
            .application_info(&app_info)
            .enabled_layer_names(&layer_name)
            .enabled_extension_names(&instance_extensions);

        let instance = unsafe {
            entry
                .create_instance(&instance_create_info, None)
                .expect("failed to create Vulkan instance!")
        };

        let mut debug_instance = ash::ext::debug_utils::Instance::new(&entry, &instance);

        let mut debug_message =
            unsafe { debug_instance.create_debug_utils_messenger(&debug_create_info, None)? };

        Ok((
            entry,
            instance,
            DebugUtils {
                instance: debug_instance,
                message: debug_message,
            },
        ))
    }

    fn create_vulkan_surface(
        window: &Window,
        entry: &Entry,
        instance: &Instance,
    ) -> Result<Surface, Box<dyn Error>> {
        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        };

        let surface_loader = ash::khr::surface::Instance::new(entry, instance);

        Ok(Surface {
            raw: surface,
            loader: surface_loader,
        })
    }

    fn create_vulkan_device(
        instance: &Instance,
        surface: &Surface,
    ) -> Result<Device, Box<dyn Error>> {
        let (physical_device, physical_device_properties) = {
            let physical_devices = unsafe { instance.enumerate_physical_devices()? };

            physical_devices
                .into_iter()
                .map(|device| {
                    let device_properties =
                        unsafe { instance.get_physical_device_properties(device) };
                    (device, device_properties)
                })
                .max_by_key(|(_, properties)| match properties.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 3,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 2,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 1,
                    _ => 0,
                })
                .expect("No physical device found")
        };

        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let (graphic_family_index, transfer_family_index) = {
            let mut found_graphic = None;

            let mut found_transfer = None;

            for (i, queue_family) in queue_family_properties.iter().enumerate() {
                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && unsafe {
                        surface.loader.get_physical_device_surface_support(
                            physical_device,
                            i as u32,
                            surface.raw,
                        )?
                    }
                {
                    found_graphic = Some(i as u32);
                }

                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER)
                    && (found_transfer.is_none()
                        || !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                {
                    found_transfer = Some(i as u32);
                }
            }
            (found_graphic.unwrap(), found_transfer.unwrap())
        };

        let priorities: [f32; 1] = [1.0];

        let queue_infos = [
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(graphic_family_index)
                .queue_priorities(&priorities),
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(transfer_family_index)
                .queue_priorities(&priorities),
        ];

        let device_extensions: Vec<*const c_char> = vec![vk::KHR_SWAPCHAIN_NAME.as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&device_extensions);

        let logical_device =
            unsafe { instance.create_device(physical_device, &device_create_info, None)? };

        let graphic_queue = unsafe { logical_device.get_device_queue(graphic_family_index, 0) };

        let transfer_queue = unsafe { logical_device.get_device_queue(transfer_family_index, 0) };

        Ok(Device {
            graphic_queue,
            transfer_queue,
            graphic_family_index,
            transfer_family_index,
            logical: logical_device,
            physical: physical_device,
        })
    }

    fn create_vulkan_swapchain(
        window: &Window,
        instance: &Instance,
        surface: &Surface,
        device: &Device,
    ) -> Result<Swapchain, Box<dyn Error>> {
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

        Ok(Swapchain {
            raw: swapchain,
            loader: swapchain_loader,
            images_view: swapchain_images_views,
        })
    }
}

impl Drop for VulkanCore {
    fn drop(&mut self) {
        unsafe {
            for image in self.swapchain.images_view.iter() {
                self.device.logical.destroy_image_view(*image, None);
            }

            self.swapchain
                .loader
                .destroy_swapchain(self.swapchain.raw, None);

            self.surface.loader.destroy_surface(self.surface.raw, None);

            self.debug
                .instance
                .destroy_debug_utils_messenger(self.debug.message, None);

            self.device.logical.destroy_device(None);

            self.instance.destroy_instance(None);
        }
    }
}
