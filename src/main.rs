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
        .add_systems(Startup, visualization::scene::setup_scene)
        .add_systems(Update, visualization::camera::camera_control)
        .add_systems(
            Update,
            (
                update_result,
                state::advance_animation,
                update_plane,
                update_solution_marker,
            )
                .chain(),
        )
        .add_systems(EguiPrimaryContextPass, equation_ui_system)
        .run();
}
