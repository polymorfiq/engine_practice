use crate::{Model, Modelable, Vertex, Transformable};
use crate::d2::Rectangle;

pub struct Cube {
    vertices: [Vertex; 24],
    indices: [usize; 42]
}

const RECT_V: usize = 4;
const RECT_I: usize = 6;
const NUM_RECTS: usize = 6;

impl Cube {
    pub fn new() -> Self {
        let left = Rectangle::new()
            .rotate(0.0, -90.0, 0.0)
            .translate(-0.5, 0.0, 0.0);

        let right = Rectangle::new()
            .rotate(0.0, 90.0, 0.0)
            .translate(0.5, 0.0, 0.0);

        let front = Rectangle::new()
            .rotate(0.0, 0.0, 0.0)
            .translate(0.0, 0.0, 0.5);

        let back = Rectangle::new()
            .rotate(0.0, 180.0, 0.0)
            .translate(0.0, 0.0, -0.5);

        let top = Rectangle::new()
            .rotate(-90.0, 0.0, 0.0)
            .translate(0.0, -0.5, 0.0);

        let bottom = Rectangle::new()
            .rotate(90.0, 0.0, 0.0)
            .translate(0.0, 0.5, 0.0);

        let models = [
            left,
            right,
            front,
            back,
            bottom,
            top
        ];

        let mut i = 0;
        let mut v = 0;
        let mut indices = [0; (RECT_I*NUM_RECTS)+NUM_RECTS];
        let mut vertices = [Default::default(); (RECT_V*NUM_RECTS)];
        for model in models {
            for idx in model.indices {
                indices[i] = idx + v;
                i += 1;
            }

            // Primitive Reset indicator
            indices[i] = 0xFFFFFFFF;
            i += 1;

            for vtx in model.vertices {
                vertices[v] = vtx;
                v += 1;
            }
        }

        Self {
            vertices,
            indices
        }
    }
}

impl Modelable<24, 42> for Cube {
    fn model(&self) -> Model<24, 42> {
        Model {
            vertices: self.vertices,
            indices: self.indices
        }
    }
}