use nalgebra::{Matrix4, Scale3, Translation3};

pub struct Transform {
    position: Translation3<f32>,
    rotation: Rotation,
    scale: Scale3<f32>,
    matrix: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Translation3::new(0.0, 0.0, 0.0),
            rotation: Rotation::new(0.0, 0.0, 0.0),
            scale: Scale3::new(0.0, 0.0, 0.0),
            matrix: Matrix4::identity(),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Translation3::new(x, y, z);
        self.update_matrix();
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = Rotation::new(x, y, z);
        self.update_matrix();
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = Scale3::new(x, y, z);
        self.update_matrix();
    }

    pub fn update_matrix(&mut self) {
        let p = self.position.to_homogeneous();
        let s = self.scale.to_homogeneous();
        let r = self.rotation.matrix();
        self.matrix = (p * r * s).try_inverse().unwrap();
    }

    pub fn as_matrix(&self) -> &Matrix4<f32> {
        &self.matrix
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        let sx = self.x.sin();
        let cx = self.x.cos();
        let sy = self.y.sin();
        let cy = self.y.cos();
        let sz = self.z.sin();
        let cz = self.z.cos();

        Matrix4::new(
            // z
            cz, -sz, 0.0, 0.0, sz, cz, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ) * Matrix4::new(
            // y
            cy, 0.0, sy, 0.0, 0.0, 1.0, 0.0, 0.0, -sy, 0.0, cy, 0.0, 0.0, 0.0, 0.0, 1.0,
        ) * Matrix4::new(
            // x
            1.0, 0.0, 0.0, 0.0, 0.0, cx, -sx, 0.0, 0.0, sx, cx, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}
