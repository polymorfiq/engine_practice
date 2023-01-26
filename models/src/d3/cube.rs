extern crate std;
use crate::{Model, Modelable, Vertex, Transformable};
use crate::d2::Rectangle;

pub struct Cube {
    vertices: [Vertex; 24],
    indices: [usize; 36]
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

        let vertices: [Vertex; RECT_V*NUM_RECTS] = unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            let dest = result.as_mut_ptr() as *mut Vertex;
            std::ptr::copy_nonoverlapping(left.vertices.as_ptr(), dest, RECT_V);
            std::ptr::copy_nonoverlapping(right.vertices.as_ptr(), dest, RECT_V);
            std::ptr::copy_nonoverlapping(top.vertices.as_ptr(), dest, RECT_V);
            std::ptr::copy_nonoverlapping(bottom.vertices.as_ptr(), dest, RECT_V);
            std::ptr::copy_nonoverlapping(front.vertices.as_ptr(), dest, RECT_V);
            std::ptr::copy_nonoverlapping(back.vertices.as_ptr(), dest, RECT_V);
            result.assume_init()
        };

        let mut indices = [0; RECT_I*NUM_RECTS];
        for i in 0..RECT_I { indices[i] = left.indices[i] }
        for i in 0..RECT_I { indices[i + (RECT_I * 1)] = right.indices[i] + (RECT_V * 1) }
        for i in 0..RECT_I { indices[i + (RECT_I * 2)] = top.indices[i] + (RECT_V * 2) }
        for i in 0..RECT_I { indices[i + (RECT_I * 3)] = bottom.indices[i] + (RECT_V * 3) }
        for i in 0..RECT_I { indices[i + (RECT_I * 4)] = front.indices[i] + (RECT_V * 4) }
        for i in 0..RECT_I { indices[i + (RECT_I * 5)] = back.indices[i] + (RECT_V * 5) }

        Self {
            vertices,
            indices
        }
    }
}

impl Modelable<24, 36> for Cube {
    fn model(&self) -> Model<24, 36> {
        Model {
            vertices: self.vertices,
            indices: self.indices
        }
    }
}