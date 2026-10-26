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
        .init_resource::<state::EquationSet>()
        .init_resource::<AppliedEquation>()
        .init_resource::<DraftEquation>()
        .init_resource::<CalculationState>()
        .init_resource::<StepState>()
        .init_resource::<visualization::scene::ActiveScene>()
        .init_resource::<visualization::cube::CubeAnimation>()
        .add_systems(Startup, visualization::scene::setup_scene)
        .add_systems(
            Update,
            (
                visualization::plane::move_plane.run_if(visualization::scene::plane_is_active),
                visualization::camera::camera_control,
                visualization::cube::animate_cube,
            ),
        )
        .add_systems(
            Update,
            (update_result, update_plane, update_solution_marker).chain(),
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
