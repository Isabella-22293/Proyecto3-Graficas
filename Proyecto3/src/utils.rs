use crate::color::Color;

pub fn lerp_color(c1: &Color, c2: &Color, t: f32) -> Color {
    let r = c1.r as f32 * (1.0 - t) + c2.r as f32 * t;
    let g = c1.g as f32 * (1.0 - t) + c2.g as f32 * t;
    let b = c1.b as f32 * (1.0 - t) + c2.b as f32 * t;

    Color::new(r as u8, g as u8, b as u8)
}
