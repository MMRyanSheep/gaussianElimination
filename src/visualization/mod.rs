use crate::state::{PlaneId, Settings};

pub mod axes;
pub mod plane;
pub mod scene;

use bevy::prelude::*;

#[derive(Component)]
pub struct Cube;

pub fn setup_scene(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    axes::spawn_axes(&mut commands, &mut mesh, &mut materials);
    plane::spawn_plane(&mut commands, &mut mesh, &mut materials);
    // commands.spawn((
    //     Cube,
    //     Mesh3d(mesh.add(Cuboid::new(2.0, 2.0, 2.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    //     Transform::default(),
    // ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(7.0, 7.0, 7.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 7.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// pub fn box_move(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
//     for mut transformation in &mut query {
//         transformation.translation.x += 1.0 * time.delta_secs();
//     }
// }
