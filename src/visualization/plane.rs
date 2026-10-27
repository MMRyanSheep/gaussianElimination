use bevy::prelude::*;

use crate::state::{AppliedEquation, LinearEquation, PlaneId};

pub fn equation_to_transform(eq: &LinearEquation) -> Option<Transform> {
    let normal = Vec3::new(eq.a, eq.b, eq.c);

    let normal_dot = normal.dot(normal);

    if !normal_dot.is_finite() || normal_dot < 1e-12 || !eq.d.is_finite() {
        return None;
    }

    let position = normal * (eq.d / normal_dot);

    let direction = normal.normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    Some(Transform {
        translation: position,
        rotation,
        ..default()
    })
}

pub fn update_plane(
    equations: Res<AppliedEquation>,
    mut plane: Query<(&PlaneId, &mut Transform, &mut Visibility)>,
) {
    if !equations.is_changed() {
        return;
    }
    for (id, mut transform, mut visibility) in &mut plane {
        let equation = equations.equations[id.0];

        match equation_to_transform(&equation) {
            Some(new_transform) => {
                *transform = new_transform;
                *visibility = Visibility::Visible;
            }
            None => *visibility = Visibility::Hidden,
        }
    }
}

pub fn spawn_plane(
    commands: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    material: &mut ResMut<Assets<StandardMaterial>>,
    equations: Res<AppliedEquation>,
) {
    let colors = [
        Color::srgba(1.0, 0.0, 1.0, 0.35),
        Color::srgba(0.0, 0.0, 1.0, 0.35),
        Color::srgba(0.0, 1.0, 0.0, 0.35),
    ];
    let mesh = mesh.add(Plane3d::default().mesh().size(10.0, 10.0));
    for (id, eq) in equations.equations.iter().enumerate() {
        let Some(transform) = equation_to_transform(eq) else {
            continue;
        };
        let material = material.add(StandardMaterial {
            base_color: colors[id],
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            double_sided: true,
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            PlaneId(id),
            transform,
        ));
    }
}
