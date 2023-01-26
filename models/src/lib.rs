#![no_std]
pub mod d2;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 3],
}

#[derive(Copy, Clone, Debug)]
pub struct Model<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [usize],
}