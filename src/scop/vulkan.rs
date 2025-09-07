use ash::Entry;

use winit::window::Window;

use std::error::Error;

mod device;
use crate::scop::vulkan::device::Device;

mod instance;
use crate::scop::vulkan::instance::Instance;

mod surface;
use crate::scop::vulkan::surface::Surface;

mod swapchain;
use crate::scop::vulkan::swapchain::Swapchain;

mod renderpass;
use crate::scop::vulkan::renderpass::RenderPass;

pub struct VulkanSetup {
    pub instance: Instance,
    pub surface: Surface,
    pub device: Device,
    pub swapchain: Swapchain,
    pub renderpass: RenderPass,
}

impl VulkanSetup {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { Entry::load()? };
        let instance = Instance::new(window, &entry)?;
        let surface = Surface::new(window, &entry, &instance.raw)?;
        let device = Device::new(&instance.raw, &surface)?;
        let mut swapchain = Swapchain::new(window, &instance.raw, &surface, &device)?;
        let renderpass = RenderPass::new(&device, &swapchain)?;
        swapchain.create_framebuffers(&device, &renderpass)?;
        Ok(Self {
            instance,
            surface,
            device,
            swapchain,
            renderpass,
        })
    }
}

impl Drop for VulkanSetup {
    fn drop(&mut self) {
        self.swapchain.clean(&self.device);
        self.surface.clean();
        self.device.clean();
        self.instance.clean();
    }
}
