use bevy::prelude::*;

use crate::state::{EquationSet, LinearEquation, PlaneId};

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

pub fn move_plane(
    mut equations: ResMut<EquationSet>,
    key: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlaneId, &mut Transform)>,
) {
    if !key.just_pressed(KeyCode::Space) {
        return;
    }
    let eq = &mut equations.equations[1];

    eq.d = if eq.d == 3.0 { 5.0 } else { 3.0 };
    for (id, mut trans) in &mut query {
        if id.0 != 1 {
            continue;
        }

        if let Some(new_trans) = equation_to_transform(eq) {
            *trans = new_trans;
            //println!("done");
        }
    }
}

pub fn spawn_plane(
    commands: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    material: &mut ResMut<Assets<StandardMaterial>>,
    equations: Res<EquationSet>,
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
