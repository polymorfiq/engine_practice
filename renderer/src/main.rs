
extern crate ash;
use ash::vk;
use std::ffi::CString;

mod window;
use window::Window;

mod gpu;
use gpu::System;

fn main() {
    let app_name_str = CString::new("my_renderer_app").expect("Unable to wrap app_name in CString");
    let engine_name_str = CString::new("my_renderer_engine").expect("Unable to wrap engine_name in CString");


    let entry = unsafe { ash::Entry::load().expect("Error loading ash::Entry") };
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name_str)
        .application_version(0)
        .engine_name(&engine_name_str)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));
        
    let system = System::new(entry, app_info);
    let window = Window::new("My Window", 800, 600);

    let instance = system.instance(window);
    let surface = system.surface(&instance);
    let device = instance.device_for_surface(&surface).expect("Device not found");

    instance.window.handle_events(|| {

    });

    device.cleanup();
    instance.cleanup_surfaces(&[surface]);
    instance.cleanup();
    system.cleanup();

    println!("Cleaned up!!");
}