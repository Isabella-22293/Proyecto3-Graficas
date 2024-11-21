pub struct Planet {
    pub name: String,
    pub position: Vec3,
    pub scale: f32,
    pub rotation_speed: f32,
    pub rotation_angle: f32,
    pub shader_type: ShaderType,
}

pub enum ShaderType {
    Star,
    RockyPlanet,
    GasGiant,
    Atmosphere,
    Surface,
    Rings,
    Moon,
}

impl Planet {
    pub fn new(name: &str, position: Vec3, scale: f32, rotation_speed: f32, shader_type: ShaderType) -> Self {
        Planet {
            name: name.to_string(),
            position,
            scale,
            rotation_speed,
            rotation_angle: 0.0,
            shader_type,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation_angle = (self.rotation_angle + self.rotation_speed * delta_time) % 360.0;
    }

    pub fn model_matrix(&self) -> Mat4 {
        nalgebra_glm::scaling(&Vec3::new(self.scale, self.scale, self.scale))
            * nalgebra_glm::translation(&self.position)
            * nalgebra_glm::rotation(self.rotation_angle.to_radians(), &Vec3::y())
    }
}
