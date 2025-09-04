use ash::{vk, Entry};

use std::error::Error;

use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use winit::window::Window;

pub struct Surface {
    pub raw: vk::SurfaceKHR,
    pub loader: ash::khr::surface::Instance,
}

impl Surface {
    pub fn new(
        window: &Window,
        entry: &Entry,
        instance: &ash::Instance,
    ) -> Result<Self, Box<dyn Error>> {
        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        };

        let surface_loader = ash::khr::surface::Instance::new(entry, instance);

        Ok(Self {
            raw: surface,
            loader: surface_loader,
        })
    }
}
