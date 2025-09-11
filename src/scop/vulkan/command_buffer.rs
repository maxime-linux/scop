use ash::vk;

use std::error::Error;

use crate::scop::vulkan::device::Device;

use crate::scop::vulkan::swapchain::Swapchain;

use crate::scop::vulkan::renderpass::RenderPass;

use crate::scop::vulkan::pipeline::Pipeline;

use crate::scop::vulkan::pools::Pools;

pub struct CommandBuffer {
    pub raw: Vec<vk::CommandBuffer>,
}

impl CommandBuffer {
    pub fn new(
        pools: &Pools,
        device: &Device,
        renderpass: &RenderPass,
        swapchain: &Swapchain,
        pipeline: &Pipeline,
        amount: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(pools.graphic)
            .command_buffer_count(amount as u32);

        let command_buffers = unsafe {
            device
                .logical
                .allocate_command_buffers(&command_buffer_allocate_info)?
        };

        for (i, command_buffer) in command_buffers.iter().enumerate() {
            let command_buffer_begin_info = vk::CommandBufferBeginInfo::default();

            unsafe {
                device
                    .logical
                    .begin_command_buffer(*command_buffer, &command_buffer_begin_info)?
            };

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.8, 1.0],
                },
            }];

            let renderpass_begin_info = vk::RenderPassBeginInfo::default()
                .render_pass(renderpass.raw)
                .framebuffer(swapchain.framebuffers[i])
                .render_area(
                    vk::Rect2D::default()
                        .offset(vk::Offset2D::default())
                        .extent(swapchain.extent),
                )
                .clear_values(&clear_values);

            unsafe {
                device.logical.cmd_begin_render_pass(
                    *command_buffer,
                    &renderpass_begin_info,
                    vk::SubpassContents::INLINE,
                );

                device.logical.cmd_bind_pipeline(
                    *command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.raw,
                );

                device.logical.cmd_draw(*command_buffer, 1, 1, 0, 0);
                device.logical.cmd_end_render_pass(*command_buffer);
                device.logical.end_command_buffer(*command_buffer)?
            }
        }

        Ok(Self {
            raw: command_buffers,
        })
    }
}
