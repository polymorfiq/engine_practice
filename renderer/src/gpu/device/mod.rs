extern crate ash;

use ash::vk;
use super::ids::SurfaceID;

pub struct Device {
    surface_id: SurfaceID,
    pdevice: vk::PhysicalDevice,
    device: ash::Device
}

impl Device {
    pub(super) fn new(surface_id: SurfaceID, pdevice: vk::PhysicalDevice, device: ash::Device) -> Self {
        Self {
            surface_id,
            pdevice,
            device
        }
    }

    pub fn surface_format(&self) -> vk::SurfaceFormatKHR {
        let surface = self.surface_id.surface();
        let instance = surface.instance_id.instance();

        unsafe {
            instance.surface_loader
                .get_physical_device_surface_formats(self.pdevice, surface.surface)
                .unwrap()[0]
        }
    }

    pub fn surface_capabilities(&self) -> vk::SurfaceCapabilitiesKHR {
        let surface = self.surface_id.surface();
        let instance = surface.instance_id.instance();

        unsafe {
            instance.surface_loader
                .get_physical_device_surface_capabilities(self.pdevice, surface.surface)
                .unwrap()
        }
    }

    pub fn cleanup(&self) {
        unsafe { self.device.destroy_device(None); }
    }
}