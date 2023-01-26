use crate::{Model, Vertex};

static VERTICES: [Vertex; 4] = [
    Vertex {pos: [-0.5, -0.5, 0.0]},
    Vertex {pos: [-0.5, 0.5, 0.0]},
    Vertex {pos: [0.5, -0.5, 0.0]},
    Vertex {pos: [0.5, 0.5, 0.0]},
];

static INDICES: [usize; 6] = [0, 1, 2, 1, 2, 3];

pub struct Rectangle<'a> {
    model: Model<'a>
}

impl<'a> Rectangle<'a> {
    pub fn new() -> Self {
        Self {
            model: Model {
                vertices: &VERTICES,
                indices: &INDICES
            }
        }
    }
}

impl<'a> core::convert::Into<Model<'a>> for Rectangle<'a> {
    fn into(self) -> Model<'a> {
        self.model
    }
}