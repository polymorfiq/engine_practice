use crate::{Model, Vertex, Vector};

pub struct Cube {
    vertices: [Vertex; 4],
    indices: [usize; 6]
}

impl Cube {
    pub fn new(normal: [f32; 3]) -> Self {
        let vertices = [
            Vertex {pos: Vector::vector([-0.5, -0.5, 0.0]), normal: Vector::vector(normal)},
            Vertex {pos: Vector::vector([-0.5, 0.5, 0.0]), normal: Vector::vector(normal)},
            Vertex {pos: Vector::vector([0.5, -0.5, 0.0]), normal: Vector::vector(normal)},
            Vertex {pos: Vector::vector([0.5, 0.5, 0.0]), normal: Vector::vector(normal)},
        ];

        let indices = [0, 1, 2, 1, 2, 3];

        Self {
            vertices,
            indices
        }
    }
}

impl Model for Cube {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) ->  &[usize] {
        &self.indices
    }
}