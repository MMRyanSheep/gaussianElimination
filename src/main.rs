mod math;
mod state;
mod ui;
mod visualization;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_panorbit_camera::*;
use state::update_result;
use visualization::plane::update_plane;
use visualization::solution::update_solution_marker;

use crate::{state::StepState, ui::equation_ui_system};
use state::{AppliedEquation, CalculationState, DraftEquation};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin, EguiPlugin::default()))
        .init_resource::<AppliedEquation>()
        .init_resource::<DraftEquation>()
        .init_resource::<CalculationState>()
        .init_resource::<StepState>()
        .init_resource::<visualization::scene::ActiveScene>()
        .init_resource::<visualization::cube::CubeAnimation>()
        .init_resource::<visualization::volume::VolumeSceneState>()
        .add_systems(Startup, visualization::scene::setup_scene)
        .add_systems(Update, visualization::camera::camera_control)
        .add_systems(
            Update,
            (
                update_result,
                state::advance_animation.run_if(visualization::scene::plane_is_active),
                visualization::cube::animate_cube,
                update_plane,
                update_solution_marker,
                visualization::volume::advance_volume_animation,
                visualization::volume::sync_volume_scene,
                visualization::volume::update_volume_scene,
            )
                .chain(),
        )
        .add_systems(Update, visualization::scene::sync_scene_visibility)
        .add_systems(
            EguiPrimaryContextPass,
            (
                ui::scene_ui_system,
                equation_ui_system.run_if(visualization::scene::plane_is_active),
            )
                .chain(),
        )
        .run();
}
