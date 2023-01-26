#![no_std]
pub mod d2;
pub mod d3;

pub use linalg::Vector;
use world::ModelMatrix;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: Vector<f32, 3>,
    pub normal: Vector<f32, 3>,
}

pub trait Modelable<const V: usize, const I: usize> {
    fn model(&self) -> Model<V, I>;
}

pub trait Renderable {
    fn vertices(&self) -> &[Vertex];
    fn indices(&self) -> &[usize];
}

#[derive(Copy, Clone, Debug)]
pub struct Model<const V: usize, const I: usize> {
    pub vertices: [Vertex; V],
    pub indices: [usize; I]
}

impl<const V: usize, const I: usize> Modelable<V, I> for Model<V, I> {
    fn model(&self) -> Model<V, I> {
        *self
    }
}

impl<const V: usize, const I: usize> Renderable for Model<V, I> {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[usize] {
        &self.indices
    }
}

pub trait Transformable<const V: usize, const I: usize> {
    fn scale(self, x: f32, y: f32, z: f32) -> Model<V, I>;
    fn translate(self, x: f32, y: f32, z: f32) -> Model<V, I>;
    fn rotate(self, x: f32, y: f32, z: f32) -> Model<V, I>;
}

impl<const V: usize, const I: usize, M: Modelable<V, I>> Transformable<V, I> for M {
    fn scale(self, x: f32, y: f32, z: f32) -> Model<V, I> {
        let m = ModelMatrix {
            scale: (x, y, z),
            rotation: (0.0, 0.0, 0.0),
            translation: (0.0, 0.0, 0.0)
        };

        let model = self.model();
        let vertices = model.vertices.map(|v| {
            Vertex {
                pos: (m.matrix() * Vector::vec4(v.pos, 1.0)).vec3(),
                normal: (m.matrix() * Vector::vec4(v.normal, 1.0)).vec3()
            }
        });

        Model {
            vertices,
            indices: model.indices
        }
    }

    fn rotate(self, x: f32, y: f32, z: f32) -> Model<V, I> {
        let m = ModelMatrix {
            scale: (1.0, 1.0, 1.0),
            rotation: (x, y, z),
            translation: (0.0, 0.0, 0.0)
        };

        let model = self.model();
        let vertices = model.vertices.map(|v| {
            Vertex {
                pos: (m.matrix() * Vector::vec4(v.pos, 1.0)).vec3(),
                normal: (m.matrix() * Vector::vec4(v.normal, 1.0)).vec3()
            }
        });

        Model {
            vertices,
            indices: model.indices
        }
    }

    fn translate(self, x: f32, y: f32, z: f32) -> Model<V, I> {
        let m = ModelMatrix {
            scale: (1.0, 1.0, 1.0),
            rotation: (0.0, 0.0, 0.0),
            translation: (x, y, z)
        };

        let model = self.model();
        let vertices = model.vertices.map(|v| {
            Vertex {
                pos: (m.matrix() * Vector::vec4(v.pos, 1.0)).vec3(),
                normal: (m.matrix() * Vector::vec4(v.normal, 1.0)).vec3()
            }
        });

        Model {
            vertices,
            indices: model.indices
        }
    }
}
