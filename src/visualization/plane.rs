use bevy::prelude::*;

use super::scene::{ActiveScene, SceneContent};
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
    active: Res<ActiveScene>,
    mut plane: Query<(&PlaneId, &mut Transform, &mut Visibility)>,
) {
    if !equations.is_changed() && !active.is_changed() {
        return;
    }
    for (id, mut transform, mut visibility) in &mut plane {
        let equation = equations.equations[id.0];

        match equation_to_transform(&equation) {
            Some(new_transform) => {
                *transform = new_transform;
                *visibility = if *active == ActiveScene::Plane {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
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
        Color::srgba(1.0, 0.48, 0.42, 0.32),
        Color::srgba(0.36, 0.60, 1.0, 0.32),
        Color::srgba(0.24, 0.86, 0.72, 0.32),
    ];
    let mesh = mesh.add(Plane3d::default().mesh().size(10.0, 10.0));
    for (id, eq) in equations.equations.iter().enumerate() {
        let transform = equation_to_transform(eq);
        let material = material.add(StandardMaterial {
            base_color: colors[id],
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            double_sided: true,
            perceptual_roughness: 0.68,
            reflectance: 0.22,
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            PlaneId(id),
            SceneContent(ActiveScene::Plane),
            transform.unwrap_or_default(),
            if transform.is_some() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
        ));
    }
}
