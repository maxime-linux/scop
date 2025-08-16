use ash::vk::{make_api_version,InstanceCreateInfo, ApplicationInfo, API_VERSION_1_3};
use ash::{Entry,Instance};


use std::error::Error;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod window;

pub struct VulkanApp {
    window: Option<Window>,
    entry: Entry,
    instance: Instance,
}

impl VulkanApp {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };
        let instance = Self::create_instance(&entry)?;
        Ok(Self {
            window: None,
            entry,
            instance,
        })
    }

    fn create_instance(entry: &Entry) -> Result<Instance, Box<dyn Error>> {
        let app_info: ApplicationInfo = ApplicationInfo::default();
        app_info.application_name(c"scop");
        app_info.application_version(make_api_version(0, 1, 0, 0));
        app_info.engine_name(c"scop_engine");
        app_info.engine_version(make_api_version(0, 1, 0, 0));
        app_info.api_version(API_VERSION_1_3);
        
        let instance_create_info = InstanceCreateInfo::default().application_info(&app_info);

        unsafe { Ok(entry.create_instance(&instance_create_info, None)?) }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("scop run!");
        let window_loop = EventLoop::new()?;
        window_loop.set_control_flow(ControlFlow::Poll);
        window_loop.run_app(self)?;
        Ok(())
    }
}
