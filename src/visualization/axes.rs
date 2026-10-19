use bevy::prelude::*;

pub fn spawn_axes(
    commands: &mut Commands,
    mesh: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let length = 10.0;
    let thickness = 0.04;
    // x
    commands.spawn((
        Mesh3d(mesh.add(Cuboid::new(length, thickness, thickness))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::default(),
    ));
    //y
    commands.spawn((
        Mesh3d(mesh.add(Cuboid::new(thickness, length, thickness))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::default(),
    ));
    //z
    commands.spawn((
        Mesh3d(mesh.add(Cuboid::new(thickness, thickness, length))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::default(),
    ));
}
