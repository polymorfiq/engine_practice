extern crate ash;

use ash::{vk, Entry, extensions::ext::DebugUtils};
use vk::{KhrPortabilityEnumerationFn, KhrGetPhysicalDeviceProperties2Fn};
use std::ffi::CStr;
use crate::window::Window;
use super::{Instance, Surface};

pub struct System<'a> {
    entry: Entry,
    app_info: vk::ApplicationInfoBuilder<'a>,
    layer_names: Vec<*const i8>,
    extension_names: Vec<*const i8>,
    create_flags: vk::InstanceCreateFlags
}

impl<'a> System<'a> {
    pub fn new<'b>(entry: Entry, app_info: vk::ApplicationInfoBuilder<'b>) -> System<'b> {
        System {
            entry,
            app_info: app_info,
            layer_names: layer_names(),
            extension_names: extensions(),
            create_flags: create_flags()
        }
    }

    pub fn instance(&self, window: Window) -> Instance {
        let mut all_extensions = vec![];
        all_extensions.extend(&self.extension_names);
        all_extensions.extend(&window.required_extensions());

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&self.app_info)
            .enabled_layer_names(&self.layer_names)
            .enabled_extension_names(&all_extensions)
            .flags(self.create_flags);

        let ash_instance: ash::Instance = unsafe {
            self.entry
                .create_instance(&create_info, None)
                .expect("Vulkan Instance creation error")
        };

        Instance::new(
            &self.entry,
            ash_instance,
            window
        )
    }

    pub fn surface(&self, instance: &Instance) -> Surface {
        Surface::new(
            &instance.window, 
            &self.entry,
            &instance
        )
    }
    
    pub fn cleanup(&self) {
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