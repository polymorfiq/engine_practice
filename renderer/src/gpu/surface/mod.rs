use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use super::ids::InstanceID;

#[derive(Clone)]
pub struct Surface {
    pub(super) surface: ash::vk::SurfaceKHR,
    pub(super) instance_id: InstanceID
}

impl Surface {
    pub(super) fn new(instance_id: InstanceID) -> Surface {
        let instance = instance_id.instance();
        let entry = instance.entry_id.entry();
        let window = &instance.window;

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance.instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
        }.expect("Unable to create Vulkan Surface");

        Surface {
            surface,
            instance_id: instance_id.clone()
        }
    }

    pub fn cleanup(&self) {
        let surface_loader = &self.instance_id.instance().surface_loader;
        unsafe { surface_loader.destroy_surface(self.surface, None) };
    }
}