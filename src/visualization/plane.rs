use bevy::prelude::*;

pub fn equation_to_transform(a: f32, b: f32, c: f32, d: f32) -> Option<Transform> {
    let normal = Vec3::new(a, b, c);

    let normal_dot = normal.dot(normal);

    if !normal_dot.is_finite() || normal_dot < 1e-12 || !d.is_finite() {
        return None;
    }

    let position = normal * (d / normal_dot);

    let direction = normal.normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    Some(Transform {
        translation: position,
        rotation,
        ..default()
    })
}

pub fn spawn_plane(
    commands: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    material: &mut ResMut<Assets<StandardMaterial>>,
) {
    let transform = equation_to_transform(3.0, 2.0, 4.0, 6.0);
    if let Some(transform) = transform {
        commands.spawn((
            Mesh3d(mesh.add(Plane3d::default().mesh().size(1.0, 1.0))),
            transform,
            MeshMaterial3d(material.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                cull_mode: None,
                ..default()
            })),
        ));
    }
}
