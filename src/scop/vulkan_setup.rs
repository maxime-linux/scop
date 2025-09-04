use ash::Entry;

use winit::window::Window;

use std::error::Error;

mod device;
use crate::scop::vulkan_setup::device::Device;

mod instance;
use crate::scop::vulkan_setup::instance::Instance;

mod surface;
use crate::scop::vulkan_setup::surface::Surface;

mod swapchain;
use crate::scop::vulkan_setup::swapchain::Swapchain;

pub struct VulkanSetup {
    pub instance: Instance,
    pub surface: Surface,
    pub device: Device,
    pub swapchain: Swapchain,
}

impl VulkanSetup {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };
        let instance = Instance::new(window, &entry)?;
        let surface = Surface::new(window, &entry, &instance.raw)?;
        let device = Device::new(&instance.raw, &surface)?;
        let swapchain = Swapchain::new(window, &instance.raw, &surface, &device)?;
        Ok(Self {
            instance,
            surface,
            device,
            swapchain,
        })
    }
}

impl Drop for VulkanSetup {
    fn drop(&mut self) {
        unsafe {
            for image in self.swapchain.images_view.iter() {
                self.device.logical.destroy_image_view(*image, None);
            }

            self.swapchain
                .loader
                .destroy_swapchain(self.swapchain.raw, None);

            self.surface.loader.destroy_surface(self.surface.raw, None);

            self.device.logical.destroy_device(None);

            self.instance
                .debug_utils
                .destroy_debug_utils_messenger(self.instance.debug_messenger, None);

            self.instance.raw.destroy_instance(None);
        };
    }
}
