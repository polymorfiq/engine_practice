extern crate ash;

use ash::vk;
use ash::extensions::{
    khr::Swapchain,
};
use crate::window::Window;

mod debugger;
use debugger::Debugger;
use super::{Device, Surface};

#[macro_use]
mod macros;

pub struct Instance {
    debugger: Debugger,
    pub(super) instance: ash::Instance,
    pub window: Window,
    surface_loader: ash::extensions::khr::Surface
}

impl Instance {
    pub(super) fn new(entry: &ash::Entry, instance: ash::Instance, window: Window) -> Instance {
        let surface_loader = ash::extensions::khr::Surface::new(entry, &instance);
        let debugger = Debugger::new(entry, &instance);
        
        Instance {
            debugger,
            surface_loader,
            instance,
            window
        }
    }

    pub fn device_for_surface(&self, surface: &Surface) -> Option<Device> {
        let dvc = unsafe { find_queue!(self.instance, self.surface_loader, surface.surface, vk::QueueFlags::GRAPHICS) };

        match dvc {
            None => None,
            Some((pdevice, graphics_queue_idx)) => {
                let features = vk::PhysicalDeviceFeatures {
                    shader_clip_distance: 1,
                    ..Default::default()
                };
                let priorities = [1.0];

                let queue_info = vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(graphics_queue_idx as u32)
                    .queue_priorities(&priorities);

                let device_ext_names = device_extension_names();
                let device_create_info = vk::DeviceCreateInfo::builder()
                    .queue_create_infos(std::slice::from_ref(&queue_info))
                    .enabled_extension_names(&device_ext_names)
                    .enabled_features(&features);

                let device: ash::Device = unsafe {
                    self.instance
                        .create_device(pdevice, &device_create_info, None)
                        .unwrap()
                };

                Some(Device::new(pdevice, device))
            }
        }
        
    }

    pub fn cleanup_surfaces(&self, surfaces: &[Surface]) {
        self.debugger.cleanup();
        
        for surface in surfaces {
            surface.cleanup(&self.surface_loader)
        }
    }

    pub fn cleanup(&self) {
        unsafe { self.instance.destroy_instance(None); }
    }
}

pub fn device_extension_names() -> Vec<*const i8> {
    [
        Swapchain::name().as_ptr(),
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        vk::KhrPortabilitySubsetFn::name().as_ptr(),
    ].to_vec()
}