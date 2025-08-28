use ash::{ext::debug_utils, khr::surface, vk, Device, Entry, Instance};

use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use winit::window::Window;

use std::ffi::{c_char, c_void, CStr};

use std::error::Error;

struct DebugUtils {
    instance: debug_utils::Instance,
    message: vk::DebugUtilsMessengerEXT,
}

pub struct VulkanCore {
    entry: Entry,
    instance: Instance,
    logical_device: Device,
    debug: DebugUtils,
    surface: vk::SurfaceKHR,
    surface_loader: surface::Instance,
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
        let entry = unsafe { Entry::load().expect("failed to create Vulkan entry!") };

        let app_info: vk::ApplicationInfo = vk::ApplicationInfo::default()
            // .application_name(c"scop")
            .application_version(vk::make_api_version(0, 1, 0, 0))
            // .engine_name(c"scop_engine")
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_3);

        let layer_name: Vec<*const c_char> = vec![c"VK_LAYER_KHRONOS_validation".as_ptr()];

        let extension_platform =
            ash_window::enumerate_required_extensions(window.display_handle()?.as_raw())?;

        let mut extensions: Vec<*const c_char> = extension_platform.to_vec();
        extensions.push(ash::ext::debug_utils::NAME.as_ptr());

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
            .enabled_extension_names(&extensions);

        let instance = unsafe {
            entry
                .create_instance(&instance_create_info, None)
                .expect("failed to create Vulkan instance!")
        };

        let mut debug_instance = ash::ext::debug_utils::Instance::new(&entry, &instance);

        let mut debug_message = unsafe {
            debug_instance
                .create_debug_utils_messenger(&debug_create_info, None)
                .expect("failed to create vulkan validation layers")
        };

        let (physical_device, physical_device_properties) = {
            let physical_devices = unsafe {
                instance
                    .enumerate_physical_devices()
                    .expect("failed to find a valid physical device")
            };
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
                .expect("no valable physical device found")
        };

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )
            .expect("failed to create surface")
        };

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let queue_family_indices = {
            let mut found_graphic = None;

            let mut found_transfer = None;

            for (i, queue_family) in queue_family_properties.iter().enumerate() {
                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && unsafe {
                        surface_loader.get_physical_device_surface_support(
                            physical_device,
                            i as u32,
                            surface,
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
                .queue_family_index(queue_family_indices.0)
                .queue_priorities(&priorities),
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family_indices.1)
                .queue_priorities(&priorities),
        ];

        let device_create_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_infos);

        let logical_device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("failed to create vulkan logical device")
        };

        let graphic_queue = unsafe { logical_device.get_device_queue(queue_family_indices.0, 0) };

        let transfer_queue = unsafe { logical_device.get_device_queue(queue_family_indices.1, 0) };

        Ok(Self {
            entry,
            instance,
            logical_device,
            debug: DebugUtils {
                instance: debug_instance,
                message: debug_message,
            },
            surface,
            surface_loader,
        })
    }
}

impl Drop for VulkanCore {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
            self.debug
                .instance
                .destroy_debug_utils_messenger(self.debug.message, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
