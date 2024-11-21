use crate::fragment::Fragment;

pub fn interpolate_fragments(start: Fragment, end: Fragment, steps: usize) -> Vec<Fragment> {
    let mut fragments = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        let position = start.vertex_position * (1.0 - t) + end.vertex_position * t;
        let intensity = start.intensity * (1.0 - t) + end.intensity * t;
        let depth = start.depth * (1.0 - t) + end.depth * t;

        fragments.push(Fragment::new(position, intensity, depth));
    }
    fragments
}
