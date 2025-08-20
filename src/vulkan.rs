use ash::{ext::debug_utils, vk, Entry, Instance};

use std::error::Error;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod instance;
mod window;

pub struct VulkanApp {
    window: Option<Window>,
    _entry: Entry,
    instance: Instance,
    _validation_layer: Option<(vk::DebugUtilsMessengerEXT, debug_utils::Instance)>,
}

impl VulkanApp {
    pub fn new() -> Self {
        let _entry = unsafe { Entry::load().expect("failed to create Vulkan entry!") };
        let instance = Self::create_instance(&_entry);
        let _validation_layer = Self::create_validation_layer(&_entry, &instance);
        Self {
            window: None,
            _entry,
            instance,
            _validation_layer,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("scop run!");
        let window_loop = EventLoop::new()?;
        window_loop.set_control_flow(ControlFlow::Poll);
        window_loop.run_app(self)?;
        Ok(())
    }
}
