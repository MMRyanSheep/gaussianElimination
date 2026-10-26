use super::scene::{ActiveScene, SceneContent};
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
        SceneContent(ActiveScene::Plane),
        Mesh3d(mesh.add(Cuboid::new(length, thickness, thickness))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.3, 0.35),
            unlit: true,
            ..default()
        })),
        Transform::default(),
    ));
    //y
    commands.spawn((
        SceneContent(ActiveScene::Plane),
        Mesh3d(mesh.add(Cuboid::new(thickness, length, thickness))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.5, 1.0),
            unlit: true,
            ..default()
        })),
        Transform::default(),
    ));
    //z
    commands.spawn((
        SceneContent(ActiveScene::Plane),
        Mesh3d(mesh.add(Cuboid::new(thickness, thickness, length))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.85, 0.5),
            unlit: true,
            ..default()
        })),
        Transform::default(),
    ));
}

pub fn spawn_cube_axes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let arrow = meshes.add(Cone::new(0.06, 0.2));
    for (name, direction, color) in [
        ("X", Vec3::X, Color::srgb(0.95, 0.3, 0.35)),
        ("Y", Vec3::Y, Color::srgb(0.35, 0.85, 0.5)),
        ("Z", Vec3::Z, Color::srgb(0.35, 0.5, 1.0)),
    ] {
        let material = materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..default()
        });
        let rotation = Quat::from_rotation_arc(Vec3::Y, direction);
        commands.spawn((
            Name::new(format!("Cube {name} axis")),
            SceneContent(ActiveScene::Cube),
            Mesh3d(meshes.add(Cuboid::new(0.012, 6.0, 0.012))),
            MeshMaterial3d(material.clone()),
            Transform::from_rotation(rotation),
            Visibility::Hidden,
        ));
        commands.spawn((
            Name::new(format!("Cube +{name} arrow")),
            SceneContent(ActiveScene::Cube),
            Mesh3d(arrow.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(direction * 3.0).with_rotation(rotation),
            Visibility::Hidden,
        ));
    }
}
