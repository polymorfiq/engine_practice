use crate::{Model, Modelable, Vertex, Vector};

pub struct Rectangle {
    vertices: [Vertex; 4],
    indices: [usize; 6]
}

impl Rectangle {
    pub fn new() -> Self {
        let normal = [0.0, 0.0, -1.0];
        let t_left = Vertex {pos: Vector::vector([-0.5, -0.5, 0.0]), normal: Vector::vector(normal)};
        let t_right = Vertex {pos: Vector::vector([-0.5, 0.5, 0.0]), normal: Vector::vector(normal)};
        let b_left = Vertex {pos: Vector::vector([0.5, -0.5, 0.0]), normal: Vector::vector(normal)};
        let b_right = Vertex {pos: Vector::vector([0.5, 0.5, 0.0]), normal: Vector::vector(normal)};

        let vertices = [t_left, t_right, b_left, b_right];
        let indices = [0, 1, 2, 1, 2, 3];

        Self {
            vertices,
            indices
        }
    }
}

impl Modelable<4, 6> for Rectangle {
    fn model(&self) -> Model<4, 6> {
        Model {
            vertices: self.vertices,
            indices: self.indices
        }
    }
}