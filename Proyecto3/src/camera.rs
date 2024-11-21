use cgmath::{Matrix4, Point3, Vector3, perspective, Deg};

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            aspect_ratio,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.position, self.target, self.up);
        let proj = perspective(Deg(self.fov), self.aspect_ratio, self.near, self.far);
        proj * view
    }
}
