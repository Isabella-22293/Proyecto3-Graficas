use crate::fragment::Fragment;

pub fn barycentric_interpolation(
    v0: Fragment,
    v1: Fragment,
    v2: Fragment,
    weights: (f32, f32, f32),
) -> Fragment {
    let position = v0.vertex_position * weights.0
        + v1.vertex_position * weights.1
        + v2.vertex_position * weights.2;

    let intensity = v0.intensity * weights.0
        + v1.intensity * weights.1
        + v2.intensity * weights.2;

    let depth = v0.depth * weights.0
        + v1.depth * weights.1
        + v2.depth * weights.2;

    Fragment::new(position, intensity, depth)
}
