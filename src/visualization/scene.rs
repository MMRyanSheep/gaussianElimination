use bevy::prelude::*;

pub fn init_scene(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(mesh.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add((Color::srgb(0.7, 0.5, 0.5)))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(mesh.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(7.0, 7.0, 7.0),
    ));
}
