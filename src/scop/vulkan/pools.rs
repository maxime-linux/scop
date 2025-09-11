use ash::vk;

use std::error::Error;

use crate::scop::vulkan::device::Device;

pub struct Pools {
    pub graphic: vk::CommandPool,
    pub transfer: vk::CommandPool,
}

impl Pools {
    pub fn new(device: &Device) -> Result<Self, Box<dyn Error>> {
        let graphics_command_pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(device.graphic_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let graphics_command_pool = unsafe {
            device
                .logical
                .create_command_pool(&graphics_command_pool_info, None)?
        };

        let transfer_command_pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(device.transfer_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let transfer_command_pool = unsafe {
            device
                .logical
                .create_command_pool(&transfer_command_pool_info, None)?
        };

        Ok(Self {
            graphic: graphics_command_pool,
            transfer: transfer_command_pool,
        })
    }

    pub fn clean(&self, device: &Device) {
        unsafe {
            device.logical.destroy_command_pool(self.graphic, None);
            device.logical.destroy_command_pool(self.transfer, None);
        }
    }
}
