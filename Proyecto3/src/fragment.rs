use nalgebra_glm::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    pub vertex_position: Vec3, // Posición del vértice
    pub intensity: f32,        // Intensidad de iluminación o color
    pub depth: f32,            // Profundidad relativa del fragmento
}

impl Fragment {
    pub fn new(vertex_position: Vec3, intensity: f32, depth: f32) -> Self {
        Self {
            vertex_position,
            intensity,
            depth,
        }
    }
}
