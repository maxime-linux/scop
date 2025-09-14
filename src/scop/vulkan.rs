use winit::window::Window;

use ash::vk;

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

pub struct Vulkan {
    pub instance: Instance,
    pub surface: Surface,
    pub device: Device,
    pub swapchain: Swapchain,
    pub renderpass: RenderPass,
    pub pipeline: Pipeline,
    pub pools: Pools,
    pub command_buffers: CommandBuffer,
}

impl Vulkan {
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

    pub fn draw(&mut self) {
        self.swapchain.current_image =
            (self.swapchain.current_image + 1) % self.swapchain.amount_images as usize;

        let current_image = self.swapchain.current_image;

        let (image_index, _) = unsafe {
            self.swapchain
                .loader
                .acquire_next_image(
                    self.swapchain.raw,
                    u64::MAX,
                    self.swapchain.images_available[current_image],
                    vk::Fence::null(),
                )
                .expect("failed to get image")
        };

        unsafe {
            self.device
                .logical
                .wait_for_fences(&[self.swapchain.fences[current_image]], true, u64::MAX)
                .expect("fence waiting")
        };

        unsafe {
            self.device
                .logical
                .reset_fences(&[self.swapchain.fences[current_image]])
                .expect("reset fences")
        };

        let semaphores_available = [self.swapchain.images_available[current_image]];

        let waiting_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let semaphores_finished = [self.swapchain.rendering_finished[current_image]];

        let commandbuffers = [self.command_buffers.raw[image_index as usize]];

        let submit_info = [vk::SubmitInfo::default()
            .wait_semaphores(&semaphores_available)
            .wait_dst_stage_mask(&waiting_stages)
            .command_buffers(&commandbuffers)
            .signal_semaphores(&semaphores_finished)];

        unsafe {
            self.device
                .logical
                .queue_submit(
                    self.device.graphic_queue,
                    &submit_info,
                    self.swapchain.fences[current_image],
                )
                .expect("queue submission")
        };

        let swapchains = [self.swapchain.raw];

        let indices = [image_index];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&semaphores_finished)
            .swapchains(&swapchains)
            .image_indices(&indices);

        unsafe {
            self.swapchain
                .loader
                .queue_present(self.device.graphic_queue, &present_info)
                .expect("queue representation")
        };
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            self.device
                .logical
                .device_wait_idle()
                .expect("failed to wait device idle")
        };
        self.pools.clean(&self.device);
        self.pipeline.clean(&self.device);
        self.renderpass.clean(&self.device);
        self.swapchain.clean(&self.device);
        self.device.clean();
        self.surface.clean();
        self.instance.clean();
    }
}
