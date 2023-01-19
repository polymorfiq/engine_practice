pub mod push_constants;
pub mod shaders;
pub mod viewport;
pub mod buffer;
pub mod buffer_set;
pub mod device_properties;

pub use device_properties::DeviceProperties;
pub use buffer::Buffer;
pub use buffer_set::BufferSet;

pub trait Cleanup {
    fn cleanup(&self, engine: &crate::Engine);
}