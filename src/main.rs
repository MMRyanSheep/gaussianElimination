mod math;
mod state;
mod ui;
mod visualization;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_panorbit_camera::*;

use crate::math::determinant::test;
use crate::ui::equation_ui_system;
use state::{AppliedEquation, DraftEquation};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin, EguiPlugin::default()))
        .init_resource::<state::EquationSet>()
        .init_resource::<AppliedEquation>()
        .init_resource::<DraftEquation>()
        .add_systems(Startup, visualization::setup_scene)
        .add_systems(Startup, test)
        .add_systems(
            Update,
            (
                visualization::plane::move_plane,
                visualization::camera::camera_control,
            ),
        )
        .add_systems(EguiPrimaryContextPass, equation_ui_system)
        .run();
}
