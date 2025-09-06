use std::error::Error;

use crate::scop::vulkan_setup::VulkanSetup;

mod renderpass;
use crate::scop::vulkan_core::renderpass::RenderPass;

pub struct VulkanCore {
    pub renderpass: RenderPass,
}

impl VulkanCore {
    pub fn new(vks: &VulkanSetup) -> Result<Self, Box<dyn Error>> {
        let renderpass = RenderPass::new(vks)?;
        Ok(Self { renderpass })
    }
}
