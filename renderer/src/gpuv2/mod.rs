pub mod push_constants;
pub mod shaders;
pub mod viewport;
pub mod vertex_input;
pub mod device_properties;

pub use device_properties::DeviceProperties;
pub use vertex_input::VertexInput;

pub trait Cleanup {
    fn cleanup(&self, engine: &crate::Engine);
}