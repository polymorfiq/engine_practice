pub mod push_constants;
pub mod shaders;
pub mod viewport;

pub trait Cleanup {
    fn cleanup(&self, engine: &crate::Engine);
}