use std::error::Error;

const VALIDATION_LAYERS: bool = cfg!(debug_assertions);

use ash::{ext::debug_utils, vk, Entry};

use std::ffi::{c_char, c_void, CStr};

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

fn main() -> Result<(), Box<dyn Error>> {
    let entry = unsafe { Entry::load().expect("failed to create Vulkan entry!") };

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

    let instance = unsafe {
        entry
            .create_instance(&instance_create_info, None)
            .expect("failed to create Vulkan instance!")
    };
    let mut debug_instance: Option<debug_utils::Instance> = None;
    let mut debug_message: Option<vk::DebugUtilsMessengerEXT> = None;
    let mut debug_create_info: Option<vk::DebugUtilsMessengerCreateInfoEXT> = None;
    if VALIDATION_LAYERS {
        debug_instance = Some(ash::ext::debug_utils::Instance::new(&entry, &instance));

        debug_create_info = Some(
            vk::DebugUtilsMessengerCreateInfoEXT::default()
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
                .pfn_user_callback(Some(vulkan_debug_utils_callback)),
        );

        debug_message = unsafe {
            Some(
                debug_instance
                    .unwrap()
                    .create_debug_utils_messenger(&debug_create_info.unwrap(), None)
                    .expect("failed to create vulkan validation layers"),
            )
        };

        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("failed to find a valid physical device")
        };

        let mut physical_device: Option<vk::PhysicalDevice> = None;
        let mut physical_device_properties: Option<vk::PhysicalDeviceProperties> = None;

        for i in physical_devices {
            let device_properties = unsafe { instance.get_physical_device_properties(i) };
            if device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                physical_device = Some(i);
                physical_device_properties = Some(device_properties);
                break;
            }
        }

        let queue_family_properties = unsafe {
            instance.get_physical_device_queue_family_properties(physical_device.unwrap())
        };
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
                .create_device(physical_device.unwrap(), &device_create_info, None)
                .expect("failed to create vulkan logical device")
        };
        let _graphic_queue = unsafe { logical_device.get_device_queue(queue_family_indices.0, 0) };
        let _transfer_queue = unsafe { logical_device.get_device_queue(queue_family_indices.1, 0) };

        unsafe {
            logical_device.destroy_device(None);
            instance.destroy_instance(None);
        }
    }
    Ok(())
}
