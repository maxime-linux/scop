use winit::event::WindowEvent::{CloseRequested, RedrawRequested};

use winit::{application::ApplicationHandler, window::Window};

use crate::scop::vulkan_setup::VulkanSetup;

mod vulkan_core;
mod vulkan_setup;

#[derive(Default)]
pub struct Scop {
    window: Option<Window>,
    vks: Option<VulkanSetup>,
    // vkc: Option<VulkanCore>,
}

impl ApplicationHandler for Scop {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes())
                .expect("failed to create winit window !");
            let vulkan_setup = VulkanSetup::new(&window).expect("failed to create vulkan setup");
            // let vulkan_core = VulkanCore::new(&vulkan_setup);

            self.window = Some(window);
            self.vks = Some(vulkan_setup);
            // self.vkc = Some(vulkan_core);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            RedrawRequested => {
                println!("Hello World!");
                // self.window.as_ref().unwrap().request_redraw();
            }

            CloseRequested => event_loop.exit(),

            _ => {}
        }
    }
}

impl Scop {
    pub fn new() -> Self {
        Self::default()
    }
}
