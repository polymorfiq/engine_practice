use crate::window::Window;
use super::{Device, Instance, Surface};
use super::ids::{DeviceID, EntryID, SurfaceID, InstanceID};
use std::ffi::CStr;
use ash::{vk, Entry};
use vk::{KhrPortabilityEnumerationFn, KhrGetPhysicalDeviceProperties2Fn};
use ash::extensions::{
    khr::Swapchain,
    ext::DebugUtils
};

#[macro_use]
mod macros;


pub struct System<'a> {
    entry_id: EntryID,
    app_info: vk::ApplicationInfoBuilder<'a>,
    layer_names: Vec<*const i8>,
    extension_names: Vec<*const i8>,
    create_flags: vk::InstanceCreateFlags,
    instances: Vec<InstanceID>,
    surfaces: Vec<SurfaceID>,
    devices: Vec<DeviceID>
}

impl<'a> System<'a> {
    pub fn new<'b>(entry: Entry, app_info: vk::ApplicationInfoBuilder<'b>) -> System<'b> {
        System {
            entry_id: EntryID::new(entry),
            app_info: app_info,
            layer_names: layer_names(),
            extension_names: extensions(),
            create_flags: create_flags(),
            instances: vec![],
            surfaces: vec![],
            devices: vec![]
        }
    }

    pub fn instance(&mut self, window: Window) -> InstanceID {
        let mut all_extensions = vec![];
        all_extensions.extend(&self.extension_names);
        all_extensions.extend(&window.required_extensions());

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&self.app_info)
            .enabled_layer_names(&self.layer_names)
            .enabled_extension_names(&all_extensions)
            .flags(self.create_flags);

        let ash_instance: ash::Instance = unsafe {
            self.entry_id
                .entry()
                .create_instance(&create_info, None)
                .expect("Vulkan Instance creation error")
        };

        let instance = Instance::new(
            self.entry_id.clone(),
            ash_instance,
            window
        );

        let instance_id = InstanceID::new(instance);
        self.instances.push(instance_id.clone());

        instance_id
    }

    pub fn surface(&mut self, instance_id: &InstanceID) -> SurfaceID {
        let surface = Surface::new(
            instance_id.clone(),
        );

        let surface_id = SurfaceID::new(surface);
        self.surfaces.push(surface_id.clone());
        surface_id
    }

    pub fn device(&mut self, surface_id: &SurfaceID) -> Option<DeviceID> {
        let surface = surface_id.surface();
        let instance = surface.instance_id.instance();
        let dvc = unsafe { find_queue!(instance.instance, instance.surface_loader, surface_id.surface().surface, vk::QueueFlags::GRAPHICS) };

        match dvc {
            None => None,
            Some((pdevice, graphics_queue_family_idx)) => {
                let features = vk::PhysicalDeviceFeatures {
                    shader_clip_distance: 1,
                    ..Default::default()
                };
                let priorities = [1.0];

                let queue_info = vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(graphics_queue_family_idx as u32)
                    .queue_priorities(&priorities);

                let device_ext_names = device_extension_names();
                let device_create_info = vk::DeviceCreateInfo::builder()
                    .queue_create_infos(std::slice::from_ref(&queue_info))
                    .enabled_extension_names(&device_ext_names)
                    .enabled_features(&features);

                let device: ash::Device = unsafe {
                    instance.instance
                        .create_device(pdevice, &device_create_info, None)
                        .unwrap()
                };

                let device_id = DeviceID::new(Device::new(
                    surface_id.clone(),
                    pdevice,
                    device,
                    graphics_queue_family_idx
                ));

                self.devices.push(device_id.clone());
                Some(device_id)
            }
        }
        
    }
    
    pub fn cleanup(&self) {
        for device_id in &self.devices {
            device_id.device().cleanup()
        }

        for surface_id in &self.surfaces {
            surface_id.surface().cleanup()
        }

        for instance_id in &self.instances {
            instance_id.instance().cleanup();
        }
    }
}

fn layer_names() -> Vec<*const i8>  {
    let names: Vec<&CStr> = vec![
        unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") }
    ];

    names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect()
}

fn extensions() -> Vec<*const i8> {
    let mut names: Vec<*const i8> = vec![
        DebugUtils::name().as_ptr()
    ];

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        names.push(KhrPortabilityEnumerationFn::name().as_ptr());
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        names.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
    }

    names
}

fn create_flags() -> vk::InstanceCreateFlags {
    if cfg!(any(target_os = "macos", target_os = "ios")) {
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::default()
    }
}

pub fn device_extension_names() -> Vec<*const i8> {
    vec![
        Swapchain::name().as_ptr(),
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        vk::KhrPortabilitySubsetFn::name().as_ptr(),
    ]
}