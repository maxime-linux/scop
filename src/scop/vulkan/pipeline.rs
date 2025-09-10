use ash::vk;

use std::error::Error;

use crate::scop::vulkan::device::Device;

use crate::scop::vulkan::swapchain::Swapchain;

use crate::scop::vulkan::renderpass::RenderPass;

const FRAGMENT_SHADER_BYTES: &[u8] = include_bytes!("../../../shaders/shader.frag.spv");

const VERTEX_SHADER_BYTES: &[u8] = include_bytes!("../../../shaders/shader.vert.spv");

fn u8_to_u32_slice(bytes: &[u8]) -> Vec<u32> {
    if bytes.len() % 4 != 0 {
        panic!("spv file must be aligned with 4 bytes")
    }

    bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}
pub struct Pipeline {
    raw: vk::Pipeline,
    layout: vk::PipelineLayout,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        swapchain: &Swapchain,
        renderpass: &RenderPass,
    ) -> Result<Self, Box<dyn Error>> {
        let vertex_shader = u8_to_u32_slice(VERTEX_SHADER_BYTES);

        let vextex_shader_create_info = vk::ShaderModuleCreateInfo::default().code(&vertex_shader);

        let vertex_module = unsafe {
            device
                .logical
                .create_shader_module(&vextex_shader_create_info, None)?
        };

        let fragment_shader = u8_to_u32_slice(FRAGMENT_SHADER_BYTES);

        let fragment_shader_create_info =
            vk::ShaderModuleCreateInfo::default().code(&fragment_shader);

        let fragment_module = unsafe {
            device
                .logical
                .create_shader_module(&fragment_shader_create_info, None)?
        };

        let vertex_shader_stage = vk::PipelineShaderStageCreateInfo::default()
            .name(c"main")
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_module);

        let fragment_shader_stage = vk::PipelineShaderStageCreateInfo::default()
            .name(c"main")
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_module);

        let shader_stages = vec![vertex_shader_stage, fragment_shader_stage];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default();

        let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::POINT_LIST);

        let viewports = [vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(swapchain.extent.width as f32)
            .height(swapchain.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)];

        let scissors = [vk::Rect2D::default()
            .offset(vk::Offset2D::default().x(0).y(0))
            .extent(swapchain.extent)];

        let viewport_info = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::default()
            .line_width(1.0)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .cull_mode(vk::CullModeFlags::NONE)
            .polygon_mode(vk::PolygonMode::FILL);

        let multisampler_info = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let colorblend_attachments = [vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .alpha_blend_op(vk::BlendOp::ADD)
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )];

        let colorblend_info =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&colorblend_attachments);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();

        let pipeline_layout = unsafe {
            device
                .logical
                .create_pipeline_layout(&pipeline_layout_info, None)?
        };

        let pipeline_info = [vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampler_info)
            .color_blend_state(&colorblend_info)
            .layout(pipeline_layout)
            .render_pass(renderpass.raw)
            .subpass(0)];

        let graphic_pipeline = unsafe {
            device
                .logical
                .create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_info, None)
                .expect("failed to create graphical pipeline")[0]
        };
        unsafe {
            device.logical.destroy_shader_module(fragment_module, None);
            device.logical.destroy_shader_module(vertex_module, None);
        };

        Ok(Self {
            raw: graphic_pipeline,
            layout: pipeline_layout,
        })
    }

    pub fn clean(&self, device: &Device) {
        unsafe {
            device.logical.destroy_pipeline(self.raw, None);
            device.logical.destroy_pipeline_layout(self.layout, None);
        }
    }
}
