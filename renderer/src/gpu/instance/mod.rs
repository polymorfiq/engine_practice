use crate::window::Window;

mod debugger;
use debugger::Debugger;
use super::ids::EntryID;

pub struct Instance {
    pub(super) entry_id: EntryID,
    pub(super) instance: ash::Instance,
    pub(super) surface_loader: ash::extensions::khr::Surface,
    debugger: Debugger,
    pub window: Window
}

impl Instance {
    pub(super) fn new(entry_id: EntryID, instance: ash::Instance, window: Window) -> Instance {
        let entry = entry_id.entry();
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        let debugger = Debugger::new(entry_id.clone(), &instance);
        
        Instance {
            entry_id: entry_id.clone(),
            debugger,
            surface_loader,
            instance,
            window
        }
    }

    pub fn cleanup(&self) {
        self.debugger.cleanup();
        unsafe { self.instance.destroy_instance(None); }
    }
}