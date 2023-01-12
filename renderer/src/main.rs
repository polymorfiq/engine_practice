extern crate ash;
extern crate winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::platform::run_return::EventLoopExtRunReturn;

use ash::vk;
use ash::vk::{
    KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn, KhrPortabilitySubsetFn,
};

use std::ffi::CStr;
use raw_window_handle::HasRawDisplayHandle;

struct HelloTriangleApplication {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    instance: ash::Instance
}

impl HelloTriangleApplication {
    pub fn initialize() -> Self {
        // Create window
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Setup VK App Info
        let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"My Vulkan App\0") };

        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let mut extension_names = ash_window::enumerate_required_extensions(window.raw_display_handle())
            .unwrap()
            .to_vec();

        // Required extension for Mac
        // https://vulkan-tutorial.com/en/Drawing_a_triangle/Setup/Instance#page_Encountered-VK_ERROR_INCOMPATIBLE_DRIVER
        extension_names.push(KhrPortabilityEnumerationFn::name().as_ptr());
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        extension_names.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::default()
        };

        // Global extensions - need winit extension
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&[])
            .enabled_extension_names(&extension_names)
            .flags(create_flags);

        let entry = unsafe { ash::Entry::load().expect("Entry load error") };
        
        let instance = unsafe {
            entry
            .create_instance(&create_info, None)
            .expect("Instance creation error")
        };

        Self {
            event_loop,
            window,
            instance
        }
    }

    fn main_loop(&mut self) {
        self.event_loop
            .run_return(|event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
        
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } if window_id == self.window.id() => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            });
    }

    fn cleanup(&mut self) {
        unsafe { self.instance.destroy_instance(None); }
    }
}


fn main() {
    let mut app = HelloTriangleApplication::initialize();
    app.main_loop();
    app.cleanup();
}