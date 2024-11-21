pub fn load_with_materials(filename: &str, materials: &[Material]) -> Result<Self, tobj::LoadError> {
    let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ..Default::default()
    })?;

    let meshes = models.into_iter().map(|model| {
        let mesh = model.mesh;
        let material = materials.get(mesh.material_id.unwrap_or(0))
            .cloned()
            .unwrap_or_default();

        Mesh {
            vertices: mesh.positions.chunks(3)
                .map(|v| Vec3::new(v[0], v[1], v[2]))
                .collect(),
            normals: mesh.normals.chunks(3)
                .map(|n| Vec3::new(n[0], n[1], n[2]))
                .collect(),
            texcoords: mesh.texcoords.chunks(2)
                .map(|t| Vec2::new(t[0], 1.0 - t[1]))
                .collect(),
            indices: mesh.indices,
        }
    }).collect();

    Ok(Obj { meshes })
}
