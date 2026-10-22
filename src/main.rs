mod math;
mod state;
mod visualization;

use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_panorbit_camera::*;

use crate::math::determinant::test;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .init_resource::<state::EquationSet>()
        .add_systems(Startup, visualization::setup_scene)
        .add_systems(Startup, test)
        .add_systems(
            Update,
            (
                visualization::plane::move_plane,
                visualization::camera::camera_control,
            ),
        )
        .run();
}
