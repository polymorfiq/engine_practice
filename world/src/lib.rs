use linalg::Matrix;

#[derive(Copy, Clone, Debug)]
pub struct ModelMatrix {
    pub scale: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub translation: (f32, f32, f32)
}

impl ModelMatrix {
    fn scale_matrix(&self) -> Matrix<f32, 4, 4> {
        let (scale_x, scale_y, scale_z) = self.scale;

        Matrix::new([
            [scale_x, 0.0, 0.0, 0.0],
            [0.0, scale_y, 0.0, 0.0],
            [0.0, 0.0, scale_z, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    fn translation_matrix(&self) -> Matrix<f32, 4, 4> {
        let (x, y, z) = self.translation;


        Matrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    fn rotation_matrix(&self) -> Matrix<f32, 4, 4> {
        let (r_x, r_y, r_z) = self.rotation;

        let rot_x = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r_x.cos(), -r_x.sin(), 0.0],
            [0.0, r_x.sin(), r_x.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        let rot_y = Matrix::new([
            [r_y.cos(), 0.0, r_y.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r_y.sin(), 0.0, r_y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        let rot_z = Matrix::new([
            [r_z.cos(), r_z.sin(), 0.0, 0.0],
            [-r_z.sin(), r_z.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        rot_x + rot_y + rot_z
    }

    pub fn matrix(&self) -> Matrix<f32, 4, 4> {
        self.translation_matrix() * self.rotation_matrix() * self.scale_matrix()
    }
}


impl Default for ModelMatrix {
    fn default() -> Self {
        ModelMatrix {
            scale: (1.0, 1.0, 1.0),
            translation: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0)
        }
    }
}