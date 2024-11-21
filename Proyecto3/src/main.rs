mod camera;
mod color;
mod framebuffer;
mod fragment;
mod planet;
mod shaders;
mod triangle;
mod vertex;

use planet::{Planet, ShaderType};
use nalgebra_glm::{Vec3};
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut planets = vec![
        Planet::new("Sun", Vec3::new(0.0, 0.0, 0.0), 2.0, 0.0, ShaderType::Star),
        Planet::new("Earth", Vec3::new(5.0, 0.0, 0.0), 1.0, 1.0, ShaderType::RockyPlanet),
        Planet::new("Jupiter", Vec3::new(10.0, 0.0, 0.0), 2.0, 0.5, ShaderType::GasGiant),
        Planet::new("Saturn", Vec3::new(15.0, 0.0, 0.0), 2.5, 0.4, ShaderType::Rings),
    ];

    let mut window = Window::new(
        "Solar System",
        200,
        100,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for planet in &mut planets {
            planet.update(0.016); // Simulación de 60 FPS
        }    

        // Dibujar los planetas
        window.update();
    }
}
