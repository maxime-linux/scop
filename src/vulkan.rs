use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod window;

#[derive(Default)]
pub struct VulkanApp {
    window: Option<Window>,
}

impl VulkanApp {
    pub fn new() -> Self {
        VulkanApp::default()
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("scop run!");
        let window_loop = EventLoop::new()?;
        window_loop.set_control_flow(ControlFlow::Poll);
        window_loop.run_app(self)?;
        Ok(())
    }
}
