use winit::event::WindowEvent::{CloseRequested, RedrawRequested};

use winit::{application::ApplicationHandler, window::Window};

use crate::app::vulkan::VulkanCore;

mod vulkan;

#[derive(Default)]
pub struct Scop {
    window: Option<Window>,
    vulkan_core: Option<VulkanCore>,
}

impl ApplicationHandler for Scop {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes())
                .expect("failed to create winit window !");
            let vulkan_core = VulkanCore::new(&window).expect("failed to create Vulkan Core");

            self.window = Some(window);
            self.vulkan_core = Some(vulkan_core)
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
