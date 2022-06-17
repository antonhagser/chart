use nalgebra::{Matrix4, Orthographic3};

use super::primitives::transform::Transform;

pub struct Camera2D {
    width: u32,
    height: u32,
    near: f32,
    far: f32,
    ortho: Orthographic3<f32>,
    view: Transform,
}

impl Camera2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            near: -10.0,
            far: 10.0,
            ortho: Self::_build_projection(0.0, width as f32, height as f32, 0.0, -1.0, 1.0),
            view: Transform::new(),
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.build_projection_matrix();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.build_projection_matrix();
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.build_projection_matrix();
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.build_projection_matrix();
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn get_far(&self) -> f32 {
        self.far
    }

    pub fn build_projection_matrix(&mut self) {
        self.ortho = Self::_build_projection(
            0.0,
            self.width as f32,
            self.height as f32,
            0.0,
            self.near,
            self.far,
        )
    }

    fn _build_projection(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Orthographic3<f32> {
        Orthographic3::new(left, right, bottom, top, near, far)
    }

    pub fn projection_matrix(&self) -> &Matrix4<f32> {
        &self.ortho.as_matrix()
    }

    pub fn view_matrix(&self) -> &Matrix4<f32> {
        &self.view.as_matrix()
    }

    /// Get a mutable reference to the camera2 d's view.
    #[must_use]
    pub fn view_mut(&mut self) -> &mut Transform {
        &mut self.view
    }
}
