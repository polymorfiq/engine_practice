extern crate ash;

use ash::vk;

pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory
}

impl Buffer {
    pub(super) fn new(buffer: vk::Buffer, memory: vk::DeviceMemory) -> Self {
        Self {
            buffer,
            memory
        }
    }
}