use crate::{Model, Vertex};

static VERTICES: [Vertex; 3] = [
    Vertex {pos: [0.0, 0.0, 0.0]},
    Vertex {pos: [-1.0, 1.0, 0.0]},
    Vertex {pos: [1.0, 1.0, 0.0]},
];

static INDICES: [usize; 3] = [0, 1, 2];

pub struct Triangle<'a> {
    model: Model<'a>
}

impl<'a> Triangle<'a> {
    pub fn new() -> Self {
        Self {
            model: Model {
                vertices: &VERTICES,
                indices: &INDICES
            }
        }
    }
}

impl<'a> core::convert::Into<Model<'a>> for Triangle<'a> {
    fn into(self) -> Model<'a> {
        self.model
    }
}