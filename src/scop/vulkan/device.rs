use ash::vk;

use std::error::Error;

use std::ffi::c_char;

use crate::scop::vulkan::surface::Surface;

pub struct Device {
    pub graphic_queue: vk::Queue,
    pub transfer_queue: vk::Queue,
    pub graphic_index: u32,
    pub transfer_index: u32,
    pub logical: ash::Device,
    pub physical: vk::PhysicalDevice,
}

impl Device {
    pub fn new(instance: &ash::Instance, surface: &Surface) -> Result<Self, Box<dyn Error>> {
        let (physical_device, _physical_device_properties) = {
            let physical_devices = unsafe { instance.enumerate_physical_devices()? };

            physical_devices
                .into_iter()
                .map(|device| {
                    let device_properties =
                        unsafe { instance.get_physical_device_properties(device) };
                    (device, device_properties)
                })
                .max_by_key(|(_, properties)| match properties.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 3,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 2,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 1,
                    _ => 0,
                })
                .expect("No physical device found")
        };

        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let (graphic_family_index, transfer_family_index) = {
            let mut found_graphic = None;

            let mut found_transfer = None;

            for (i, queue_family) in queue_family_properties.iter().enumerate() {
                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && unsafe {
                        surface.loader.get_physical_device_surface_support(
                            physical_device,
                            i as u32,
                            surface.raw,
                        )?
                    }
                {
                    found_graphic = Some(i as u32);
                }

                if queue_family.queue_count > 0
                    && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER)
                    && (found_transfer.is_none()
                        || !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                {
                    found_transfer = Some(i as u32);
                }
            }
            (found_graphic.unwrap(), found_transfer.unwrap())
        };

        let priorities: [f32; 1] = [1.0];

        let queue_infos = [
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(graphic_family_index)
                .queue_priorities(&priorities),
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(transfer_family_index)
                .queue_priorities(&priorities),
        ];

        let device_extensions: Vec<*const c_char> = vec![vk::KHR_SWAPCHAIN_NAME.as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&device_extensions);

        let logical_device =
            unsafe { instance.create_device(physical_device, &device_create_info, None)? };

        let graphic_queue = unsafe { logical_device.get_device_queue(graphic_family_index, 0) };

        let transfer_queue = unsafe { logical_device.get_device_queue(transfer_family_index, 0) };

        Ok(Self {
            graphic_queue,
            transfer_queue,
            graphic_index: graphic_family_index,
            transfer_index: transfer_family_index,
            logical: logical_device,
            physical: physical_device,
        })
    }
    pub fn clean(&self) {
        unsafe { self.logical.destroy_device(None) };
    }
}
