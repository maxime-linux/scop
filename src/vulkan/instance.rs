use ash::vk::{make_api_version, ApplicationInfo, InstanceCreateInfo, API_VERSION_1_3};

use ash::{Entry, Instance};

use super::VulkanApp;

impl Drop for VulkanApp {
    fn drop(&mut self) {
        println!("instance vulkan destroyed");
        unsafe { self.instance.destroy_instance(None) };
    }
}

impl VulkanApp {
    pub fn create_instance(entry: &Entry) -> Instance {
        let app_info: ApplicationInfo = ApplicationInfo::default();
        app_info.application_name(c"scop");
        app_info.application_version(make_api_version(0, 1, 0, 0));
        app_info.engine_name(c"scop_engine");
        app_info.engine_version(make_api_version(0, 1, 0, 0));
        app_info.api_version(API_VERSION_1_3);

        let instance_create_info = InstanceCreateInfo::default().application_info(&app_info);

        unsafe { entry.create_instance(&instance_create_info, None).expect("failed to create Vulkan instance!") }
    }
}
