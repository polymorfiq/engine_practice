extern crate ash;

use ash::{vk, Entry};
use crate::window::Window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use super::Instance;

#[derive(Clone)]
pub struct Surface {
    pub(super) surface: vk::SurfaceKHR
}

impl Surface {
    pub(super) fn new(window: &Window, entry: &Entry, instance: &Instance) -> Surface {
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
            surface
        }
    }

    pub fn cleanup(&self, surface_loader: &ash::extensions::khr::Surface) {
        unsafe { surface_loader.destroy_surface(self.surface, None) };
    }
}