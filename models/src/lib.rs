#![no_std]
pub mod d2;
pub mod d3;

pub use linalg::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: Vector<f32, 3>,
    pub normal: Vector<f32, 3>,
}

pub trait Model {
    fn vertices(&self) -> &[Vertex];
    fn indices(&self) -> &[usize];
}
