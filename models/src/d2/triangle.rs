use crate::{Model, Modelable, Vertex, Vector};

pub struct Triangle {
    vertices: [Vertex; 3],
    indices: [usize; 3]
}

impl Triangle {
    pub fn new() -> Self {
        let normal = [0.0, 0.0, -1.0];
        let top = Vertex {pos: Vector::vector([0.0, -0.5, 0.0]), normal: Vector::vector(normal)};
        let b_left = Vertex {pos: Vector::vector([-0.5, 0.5, 0.0]), normal: Vector::vector(normal)};
        let b_right = Vertex {pos: Vector::vector([0.5, 0.5, 0.0]), normal: Vector::vector(normal)};
        
        let vertices = [b_left, top, b_right];
        let indices = [0, 1, 2];

        Self {
            vertices,
            indices
        }
    }
}

impl Modelable<3, 3> for Triangle {
    fn model(&self) -> Model<3, 3> {
        Model {
            vertices: self.vertices,
            indices: self.indices
        }
    }
}