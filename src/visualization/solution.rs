use std::result;

use bevy::prelude::*;

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
            base_color: Color::srgb(1.0, 0.85, 0.15),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(result.x as f32, result.y as f32, result.z as f32),
        SolutionMarker,
    ));
}

pub fn update_solution_marker(
    commands: Commands,
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
