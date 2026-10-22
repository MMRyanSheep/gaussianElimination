pub mod axes;
pub mod camera;
pub mod plane;
pub mod scene;

use bevy::prelude::*;
use bevy_panorbit_camera::*;
use camera::{OrbitCamera, camera_transform_update};

use crate::state::EquationSet;

pub fn setup_scene(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    equations: Res<EquationSet>,
) {
    axes::spawn_axes(&mut commands, &mut mesh, &mut materials);
    plane::spawn_plane(&mut commands, &mut mesh, &mut materials, equations);
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
        Transform::default(),
    ));
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(8.0, 7.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));
    let orbit = OrbitCamera {
        yaw: 1.0,
        pitch: 0.9,
        radius: 30.0,
        target: Vec3::ZERO,
    };

    let mut camera_trans = Transform::default();

    camera_transform_update(&mut camera_trans, &orbit);
    //commands.spawn((orbit, camera_trans, Camera3d::default()));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

// pub fn box_move(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
//     for mut transformation in &mut query {
//         transformation.translation.x += 1.0 * time.delta_secs();
//     }
// }
