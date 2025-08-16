use ash::{Entry, Instance};

use std::error::Error;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod instance;
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

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("scop run!");
        let window_loop = EventLoop::new()?;
        window_loop.set_control_flow(ControlFlow::Poll);
        window_loop.run_app(self)?;
        Ok(())
    }
}
