extern crate ash;

use ash::vk;

pub struct Device {
    pdevice: vk::PhysicalDevice,
    device: ash::Device
}

impl Device {
    pub(super) fn new(pdevice: vk::PhysicalDevice, device: ash::Device) -> Self {
        Self {
            pdevice,
            device
        }
    }

    pub fn cleanup(&self) {
        unsafe { self.device.destroy_device(None); }
    }
}