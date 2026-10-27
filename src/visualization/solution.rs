use bevy::prelude::*;

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
            base_color: Color::srgb(1.0, 0.85, 0.15),
            unlit: true,
            ..default()
        })),
        Transform::default(),
        Visibility::Hidden,
        SolutionMarker,
    ));
}

pub fn update_solution_marker(
    solution: Res<CalculationState>,
    mut query: Query<(&mut Transform, &mut Visibility), With<SolutionMarker>>,
) {
    for (mut transform, mut visibility) in &mut query {
        match &solution.result {
            Some(result) if result.can_animate() => {
                *visibility = Visibility::Visible;
                transform.translation =
                    Vec3::new(result.x as f32, result.y as f32, result.z as f32);
            }
            _ => *visibility = Visibility::Hidden,
        }
    }
}
