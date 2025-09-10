use ash::{vk, Entry};

use std::ffi::{c_char, c_void, CStr};

use winit::window::Window;

use winit::raw_window_handle::HasDisplayHandle;

use std::error::Error;

pub struct Instance {
    pub raw: ash::Instance,
    pub debug_utils: ash::ext::debug_utils::Instance,
    pub debug_messenger: vk::DebugUtilsMessengerEXT,
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    msg_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let message = unsafe { CStr::from_ptr((*callback_data).p_message).to_string_lossy() };
    eprintln!("[DEBUG][{:?}][{:?}] {}", msg_severity, msg_type, message);
    vk::FALSE
}

impl Instance {
    pub fn new(window: &Window, entry: &Entry) -> Result<Self, Box<dyn Error>> {
        let app_info: vk::ApplicationInfo = vk::ApplicationInfo::default()
            .application_name(c"scop")
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(c"scop_engine")
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

        let debug_utils = ash::ext::debug_utils::Instance::new(&entry, &instance);

        let debug_messenger =
            unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None)? };

        Ok(Self {
            raw: instance,
            debug_utils,
            debug_messenger,
        })
    }

    pub fn clean(&self) {
        unsafe {
            self.debug_utils
                .destroy_debug_utils_messenger(self.debug_messenger, None)
        };
        unsafe { self.raw.destroy_instance(None) };
    }
}
