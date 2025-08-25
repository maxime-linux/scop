const VALIDATION_LAYERS: bool = cfg!(debug_assertions);

use ash::{ext::debug_utils, vk, Device, Entry, Instance};

use std::ffi::{c_char, c_void, CStr};

use super::VulkanApp;

impl Drop for VulkanApp {
    fn drop(&mut self) {
        if let Some(debug_utils) = &self._validation_layer {
            unsafe {
                debug_utils
                    .1
                    .destroy_debug_utils_messenger(debug_utils.0, None)
            }
            println!("vulkan validation layer destroyed");
        }
        unsafe { self.instance.destroy_instance(None) };
        println!("vulkan instance destroyed");
        unsafe {
            self.logical_device.destroy_device(None);
        }
        println!("vulkan logical device detroyed");
    }
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

impl VulkanApp {
    pub fn create_instance(entry: &Entry) -> Instance {
        let app_info: vk::ApplicationInfo = vk::ApplicationInfo::default()
            // .application_name(c"scop")
            .application_version(vk::make_api_version(0, 1, 0, 0))
            // .engine_name(c"scop_engine")
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_3);

        let layer_name: Vec<*const c_char> = if VALIDATION_LAYERS {
            vec![c"VK_LAYER_KHRONOS_validation".as_ptr()]
        } else {
            vec![]
        };

        let extension: Vec<*const c_char> = if VALIDATION_LAYERS {
            vec![ash::ext::debug_utils::NAME.as_ptr()]
        } else {
            vec![]
        };

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&layer_name)
            .enabled_extension_names(&extension);

        unsafe {
            entry
                .create_instance(&instance_create_info, None)
                .expect("failed to create Vulkan instance!")
        }
    }

    pub fn create_validation_layer(
        entry: &Entry,
        instance: &Instance,
    ) -> Option<(vk::DebugUtilsMessengerEXT, debug_utils::Instance)> {
        if !VALIDATION_LAYERS {
            return None;
        }
        let debug_utils = ash::ext::debug_utils::Instance::new(entry, instance);

        let debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
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

        unsafe {
            Some((
                debug_utils
                    .create_debug_utils_messenger(&debug_create_info, None)
                    .expect("failed to create vulkan validation layers"),
                debug_utils,
            ))
        }
    }

    pub fn get_physical_device(
        instance: &Instance,
    ) -> (vk::PhysicalDevice, vk::PhysicalDeviceProperties) {
        let devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("failed to find a valid physical device")
        };

        for device in devices {
            let device_properties = unsafe { instance.get_physical_device_properties(device) };
            if device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                return (device, device_properties);
            }
        }
        panic!("failed to find a discrete gpu");
    }

    pub fn get_logical_device(
        instance: &Instance,
        physical_device: &(vk::PhysicalDevice, vk::PhysicalDeviceProperties),
    ) -> Device {
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device.0) };
        let queue_family_indices = {
            let mut found_graphic = None;
            let mut found_transfer = None;
            for (i, queue_family) in queue_family_properties.iter().enumerate() {
                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
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
                .create_device(physical_device.0, &device_create_info, None)
                .expect("failed to create vulkan logical device")
        };
        let _graphic_queue = unsafe { logical_device.get_device_queue(queue_family_indices.0, 0) };
        let _transfer_queue = unsafe { logical_device.get_device_queue(queue_family_indices.1, 0) };
        logical_device
    }
}
