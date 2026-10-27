use bevy::prelude::*;

use super::scene::{ActiveScene, SceneContent};
use crate::state::CalculationState;

#[derive(Component)]
pub struct SolutionMarker;

pub fn spawn_solution(
    commands: &mut Commands,
    mesh: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.spawn((
        Mesh3d(mesh.add(Sphere::new(0.15))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.78, 0.28),
            emissive: LinearRgba::rgb(0.35, 0.18, 0.025),
            perceptual_roughness: 0.32,
            reflectance: 0.35,
            ..default()
        })),
        Transform::default(),
        Visibility::Hidden,
        SolutionMarker,
        SceneContent(ActiveScene::Plane),
    ));
}

pub fn update_solution_marker(
    solution: Res<CalculationState>,
    active: Res<ActiveScene>,
    mut query: Query<(&mut Transform, &mut Visibility), With<SolutionMarker>>,
) {
    for (mut transform, mut visibility) in &mut query {
        match &solution.result {
            Some(result) if result.can_animate() && *active == ActiveScene::Plane => {
                *visibility = Visibility::Visible;
                transform.translation =
                    Vec3::new(result.x as f32, result.y as f32, result.z as f32);
            }
            _ => *visibility = Visibility::Hidden,
        }
    }
}
