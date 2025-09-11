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

mod pipeline;
use crate::scop::vulkan::pipeline::Pipeline;

mod pools;
use crate::scop::vulkan::pools::Pools;

mod command_buffer;
use crate::scop::vulkan::command_buffer::CommandBuffer;

pub struct VulkanSetup {
    pub instance: Instance,
    pub surface: Surface,
    pub device: Device,
    pub swapchain: Swapchain,
    pub renderpass: RenderPass,
    pub pipeline: Pipeline,
    pub pools: Pools,
    pub command_buffers: CommandBuffer,
}

impl VulkanSetup {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { ash::Entry::load()? };
        let instance = Instance::new(window, &entry)?;
        let surface = Surface::new(window, &entry, &instance.raw)?;
        let device = Device::new(&instance.raw, &surface)?;
        let mut swapchain = Swapchain::new(window, &instance.raw, &surface, &device)?;
        let renderpass = RenderPass::new(&device, &swapchain)?;
        let pipeline = Pipeline::new(&device, &swapchain, &renderpass)?;
        let pools = Pools::new(&device)?;
        swapchain.create_framebuffers(&device, &renderpass)?;
        let command_buffers = CommandBuffer::new(
            &pools,
            &device,
            &renderpass,
            &swapchain,
            &pipeline,
            swapchain.framebuffers.len(),
        )?;

        Ok(Self {
            instance,
            surface,
            device,
            swapchain,
            renderpass,
            pipeline,
            pools,
            command_buffers,
        })
    }
}

impl Drop for VulkanSetup {
    fn drop(&mut self) {
        self.pools.clean(&self.device);
        self.pipeline.clean(&self.device);
        self.swapchain.clean(&self.device);
        self.surface.clean();
        self.device.clean();
        self.instance.clean();
    }
}
