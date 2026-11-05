use super::camera::{OrbitCamera, camera_transform_update};
use bevy::prelude::*;
use bevy_panorbit_camera::*;

use crate::state::AppliedEquation;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub enum ActiveScene {
    #[default]
    Plane,
    Cube,
    /// Cramer's rule shown as a volume ratio; spawned on demand by
    /// [`super::volume`].
    Volume,
}

#[derive(Component)]
pub struct SceneContent(pub ActiveScene);

pub fn sync_scene_visibility(
    active: Res<ActiveScene>,
    mut content: Query<
        (&SceneContent, &mut Visibility),
        (
            Without<crate::state::PlaneId>,
            Without<super::solution::SolutionMarker>,
        ),
    >,
) {
    for (scene, mut visibility) in &mut content {
        *visibility = if scene.0 == *active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub fn plane_is_active(active: Res<ActiveScene>) -> bool {
    *active == ActiveScene::Plane
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switching_scenes_only_shows_the_selected_content() {
        let mut app = App::new();
        app.init_resource::<ActiveScene>()
            .add_systems(Update, sync_scene_visibility);
        let plane = app
            .world_mut()
            .spawn((SceneContent(ActiveScene::Plane), Visibility::Inherited))
            .id();
        let cube = app
            .world_mut()
            .spawn((SceneContent(ActiveScene::Cube), Visibility::Hidden))
            .id();
        for active in [
            ActiveScene::Plane,
            ActiveScene::Cube,
            ActiveScene::Volume,
            ActiveScene::Plane,
        ] {
            *app.world_mut().resource_mut::<ActiveScene>() = active;
            app.update();
            assert_eq!(
                *app.world().get::<Visibility>(plane).unwrap(),
                if active == ActiveScene::Plane {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                }
            );
            assert_eq!(
                *app.world().get::<Visibility>(cube).unwrap(),
                if active == ActiveScene::Cube {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                }
            );
        }
    }
}

pub fn setup_scene(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    equations: Res<AppliedEquation>,
) {
    // A quiet backdrop and broad studio lighting keep translucent overlaps legible.
    commands.insert_resource(ClearColor(Color::srgb(0.035, 0.052, 0.085)));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.82, 0.89, 1.0),
        brightness: 240.0,
        ..default()
    });

    super::solution::spawn_solution(&mut commands, &mut mesh, &mut materials);

    super::axes::spawn_axes(&mut commands, &mut mesh, &mut materials);
    super::plane::spawn_plane(&mut commands, &mut mesh, &mut materials, equations);
    super::cube::spawn_cube(&mut commands, &mut mesh, &mut materials);
    super::axes::spawn_cube_axes(&mut commands, &mut mesh, &mut materials);

    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.93, 0.84),
            intensity: 350_000.0,
            range: 40.0,
            radius: 2.0,
            ..default()
        },
        Transform::from_xyz(6.0, 10.0, 8.0),
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.72, 0.84, 1.0),
            illuminance: 1_800.0,
            ..default()
        },
        Transform::from_xyz(-6.0, 3.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
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
