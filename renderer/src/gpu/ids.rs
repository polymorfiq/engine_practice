use super::{Device, Instance, Surface};
use ash::Entry;
use std::{cell::{Ref, RefCell}, rc::Rc};

#[derive(Clone)]
pub struct EntryID {
    entry: Rc<RefCell<Entry>>
}
impl EntryID {
    pub(super) fn new(entry: Entry) -> Self {
        Self{entry: Rc::new(RefCell::new(entry))}
    }

    pub(super) fn entry(&self) -> Ref<Entry> {
        (*self.entry).borrow()
    }
}

#[derive(Clone)]
pub struct InstanceID {
    instance: Rc<RefCell<Instance>>
}
impl InstanceID {
    pub(super) fn new(instance: Instance) -> Self {
        Self{instance: Rc::new(RefCell::new(instance))}
    }

    pub fn instance(&self) -> Ref<Instance> {
        (*self.instance).borrow()
    }
}

#[derive(Clone)]
pub struct SurfaceID {
    surface: Rc<RefCell<Surface>>
}
impl SurfaceID {
    pub(super) fn new(surface: Surface) -> Self {
        Self{surface: Rc::new(RefCell::new(surface))}
    }

    pub fn surface(&self) -> Ref<Surface> {
        (*self.surface).borrow()
    }
}

#[derive(Clone)]
pub struct DeviceID {
    pub(super) device: Rc<RefCell<Device>>
}
impl DeviceID {
    pub(super) fn new(device: Device) -> Self {
        Self{device: Rc::new(RefCell::new(device))}
    }

    pub fn device(&self) -> Ref<Device> {
        (*self.device).borrow()
    }
}