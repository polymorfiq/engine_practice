use crate::{Model, Vertex, Vector};

pub struct Triangle {
    vertices: [Vertex; 3],
    indices: [usize; 3]
}

impl Triangle {
    pub fn new(normal: [f32; 3]) -> Self {
        let vertices = [
            Vertex {pos: Vector::vector([0.0, 0.0, 0.0]), normal: Vector::vector(normal)},
            Vertex {pos: Vector::vector([-1.0, 1.0, 0.0]), normal: Vector::vector(normal)},
            Vertex {pos: Vector::vector([1.0, 1.0, 0.0]), normal: Vector::vector(normal)},
        ];

        let indices = [0, 1, 2];

        Self {
            vertices,
            indices
        }
    }
}

impl Model for Triangle {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) ->  &[usize] {
        &self.indices
    }
}