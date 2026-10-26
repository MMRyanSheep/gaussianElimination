use bevy::prelude::*;

use super::scene::{ActiveScene, SceneContent};
use crate::math::carmer::CramerSolution;
use crate::state::CalculationState;

#[derive(Component)]
pub struct SolutionMarker;

pub fn spawn_solution(
    commands: &mut Commands,
    mesh: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    result: CramerSolution,
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
        Transform::from_xyz(result.x as f32, result.y as f32, result.z as f32),
        SolutionMarker,
        SceneContent(ActiveScene::Plane),
    ));
}

pub fn update_solution_marker(
    _commands: Commands,
    solution: Res<CalculationState>,
    mut query: Query<&mut Transform, With<SolutionMarker>>,
) {
    for mut transform in &mut query {
        match &solution.result {
            Some(result) => {
                transform.translation =
                    Vec3::new(result.x as f32, result.y as f32, result.z as f32);
            }
            None => {}
        }
    }
}
