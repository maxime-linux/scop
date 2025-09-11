use winit::event::WindowEvent::{CloseRequested, RedrawRequested};

use winit::{application::ApplicationHandler, window::Window};

use crate::scop::vulkan::Vulkan;

mod vulkan;

#[derive(Default)]
pub struct Scop {
    window: Option<Window>,
    vulkan: Option<Vulkan>,
}

impl ApplicationHandler for Scop {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes())
                .expect("failed to create winit window !");
            let vulkan_setup = Vulkan::new(&window).expect("failed to create vulkan setup");
            self.window = Some(window);
            self.vulkan = Some(vulkan_setup);
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
                self.vulkan.as_mut().unwrap().draw();
            }

            CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(vulkan) = self.vulkan.take() {
            drop(vulkan);
        }
        if let Some(window) = self.window.take() {
            drop(window);
        }
    }
}

impl Scop {
    pub fn new() -> Self {
        Self::default()
    }
}
