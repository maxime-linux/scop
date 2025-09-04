// use ash::vk;

// use crate::scop::vulkan_setup::VulkanSetup;

// fn core(vks: &VulkanSetup) {
//     let attachment = [vk::AttachmentDescription::default()
//         // .format(
//         //     vks.surface
//         //         .raw
//         //         .get_formats(vks.device.physical)?
//         //         .first()
//         //         .unwrap()
//         //         .format,
//         // )
//         .load_op(vk::AttachmentLoadOp::CLEAR)
//         .store_op(vk::AttachmentStoreOp::STORE)
//         .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
//         .store_op(vk::AttachmentStoreOp::DONT_CARE)
//         .initial_layout(vk::ImageLayout::UNDEFINED)
//         .final_layout(vk::ImageLayout::PRESENT_SRC_KHR::PRESENT_SRC_KHR)
//         .samples(samples)];
// }
