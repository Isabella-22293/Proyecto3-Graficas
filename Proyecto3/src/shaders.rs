use crate::color::Color;
use nalgebra_glm::Vec3;
use winapi::ctypes::c_void;

pub fn star_shader(position: Vec3, time: f32) -> Color { 
    let intensity = ((time.sin() * 0.5 + 0.5) * 255.0) as f32;
    Color::new(intensity, intensity, 0.0, 1.0)
}

pub fn rocky_planet_shader(position: Vec3, time: f32) -> Color {
    let t = ((position.x + time).sin() * 0.5 + 0.5) as f32;
    Color::new(139.0, 69.0, (139.0 * t), 1.0)
}

pub fn gas_giant_shader(position: Vec3, time: f32) -> Color {
    let bands = (((position.y * 10.0).sin() * 0.5 + 0.5) * 255.0) as f32;
    Color::new(bands, 100.0, (255.0 - bands), 1.0)
}

pub fn atmosphere_shader(position: Vec3, time: f32) -> Color {
    let glow = ((time.cos() * 0.5 + 0.5) * 255.0) as f32;
    Color::new(0.0, glow, 255.0, 1.0)
}

pub fn surface_shader(position: Vec3, time: f32) -> Color {
    let t = ((position.x * 5.0 + time).sin() * 0.5 + 0.5) as f32;
    Color::new((t * 200.0), 50.0, (t * 255.0), 1.0)
}

pub fn rings_shader(position: Vec3, time: f32) -> Color {
    let ring = (((position.x * 20.0).sin() * 0.5 + 0.5) * 255.0) as f32;
    Color::new(192.0, ring, 192.0, 1.0)
}

pub fn moon_shader(position: Vec3, time: f32) -> Color {
    let phase = ((position.z + time).sin() * 0.5 + 0.5) as f32;
    Color::new((255.0 * phase), (255.0 * phase), (255.0 * phase), 1.0)
}
