#![allow(dead_code)]
#[path = "../src/math/mod.rs"]
mod math;
#[path = "../src/state.rs"]
mod state;

use bevy::prelude::*;
use math::{carmer::solver, determinant::equation_to_matrix, steps::CalculationStep};
use state::*;
use std::time::Duration;

fn app() -> App {
    let mut app = App::new();
    app.init_resource::<AppliedEquation>()
        .init_resource::<CalculationState>()
        .init_resource::<StepState>()
        .insert_resource(Time::<()>::default())
        .add_systems(Update, (update_result, advance_animation).chain());
    app.update();
    app
}

fn tick(app: &mut App, seconds: f32) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(seconds));
    app.update();
}

#[test]
fn teaching_steps_use_replaced_columns_and_signed_products() {
    let app = app();
    let steps = app.world().resource::<StepState>();
    assert_eq!(steps.steps.len(), 6);
    let original = equation_to_matrix(app.world().resource::<AppliedEquation>());
    for (index, expected) in [7.0, 7.0, 14.0, 21.0].into_iter().enumerate() {
        let CalculationStep::Determinant {
            matrix,
            replaced_col,
            products,
            result,
            ..
        } = &steps.steps[index + 1]
        else {
            panic!()
        };
        assert_eq!(*result, expected);
        assert_eq!(products.iter().sum::<f64>(), expected);
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(
                    matrix[row][col],
                    if *replaced_col == Some(col) {
                        [6.0, 3.0, 2.0][row]
                    } else {
                        original[row][col]
                    }
                );
            }
        }
    }
}

#[test]
fn playback_pauses_advances_and_finishes_without_changing_equations() {
    let mut app = app();
    app.world_mut().resource_mut::<StepState>().playing = true;
    tick(&mut app, 3.5);
    let steps = app.world().resource::<StepState>();
    assert_eq!(steps.current, 1);
    assert_eq!(steps.active_product(), 1);
    app.world_mut().resource_mut::<StepState>().playing = false;
    tick(&mut app, 9.0);
    assert_eq!(app.world().resource::<StepState>().elapsed, 1.5);
    app.world_mut().resource_mut::<StepState>().playing = true;
    tick(&mut app, 30.0);
    let steps = app.world().resource::<StepState>();
    assert_eq!(steps.current, 5);
    assert!(!steps.playing);
    assert_eq!(steps.elapsed, 2.0);
    assert_eq!(
        app.world().resource::<AppliedEquation>().equations[0].d,
        6.0
    );
}

#[test]
fn applying_singular_input_clears_old_playback_and_recovers_silently() {
    let mut app = app();
    app.world_mut().resource_mut::<StepState>().playing = true;
    tick(&mut app, 4.0);
    let repeated = LinearEquation {
        a: 1.0,
        b: 1.0,
        c: 1.0,
        d: 1.0,
    };
    for inconsistent in [false, true] {
        let mut equations = [repeated; 3];
        if inconsistent {
            equations[2].d = 2.0;
        }
        app.world_mut().resource_mut::<AppliedEquation>().equations = equations;
        tick(&mut app, 0.0);
        let steps = app.world().resource::<StepState>();
        assert!(steps.steps.is_empty());
        assert!(!steps.playing);
        assert_eq!(steps.current, 0);
        assert!(
            !app.world()
                .resource::<CalculationState>()
                .result
                .unwrap()
                .can_animate()
        );
    }
    app.world_mut().resource_mut::<AppliedEquation>().equations = default_equations();
    tick(&mut app, 0.0);
    assert_eq!(app.world().resource::<StepState>().steps.len(), 6);
    assert!(
        app.world()
            .resource::<CalculationState>()
            .result
            .unwrap()
            .can_animate()
    );
}

#[test]
fn playback_gate_is_scale_independent_and_handles_zero_rows() {
    for scale in [1e-15, 1.0, 1e15] {
        let m = [[scale, 0.0, 0.0], [0.0, scale, 0.0], [0.0, 0.0, scale]];
        let solution = solver(m, [scale, 2.0 * scale, 3.0 * scale]);
        assert!(solution.can_animate());
        assert!((solution.z - 3.0).abs() < 1e-10);
    }
    assert!(!solver([[0.0; 3]; 3], [0.0; 3]).can_animate());
}

#[test]
fn manual_navigation_resets_progress_and_speed_changes_timing() {
    let mut app = app();
    {
        let mut steps = app.world_mut().resource_mut::<StepState>();
        steps.speed = 2.0;
        steps.playing = true;
    }
    tick(&mut app, 1.5);
    assert_eq!(app.world().resource::<StepState>().current, 1);
    {
        let mut steps = app.world_mut().resource_mut::<StepState>();
        steps.select(4);
        assert_eq!(steps.elapsed, 0.0);
        assert!(!steps.playing);
        steps.select(3);
        assert_eq!(steps.current, 3);
        steps.restart();
        assert_eq!(steps.current, 0);
    }
}

#[path = "../src/visualization/plane.rs"]
mod plane;
#[path = "../src/visualization/solution.rs"]
mod solution;

#[test]
fn singular_input_hides_old_marker_and_zero_planes_then_restores_them() {
    let mut app = app();
    app.add_systems(
        Update,
        (plane::update_plane, solution::update_solution_marker).after(update_result),
    );
    let marker = app
        .world_mut()
        .spawn((
            solution::SolutionMarker,
            Transform::default(),
            Visibility::Visible,
        ))
        .id();
    let plane = app
        .world_mut()
        .spawn((PlaneId(0), Transform::default(), Visibility::Visible))
        .id();
    app.world_mut().resource_mut::<AppliedEquation>().equations = [LinearEquation {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
    }; 3];
    tick(&mut app, 0.0);
    assert_eq!(
        *app.world().get::<Visibility>(marker).unwrap(),
        Visibility::Hidden
    );
    assert_eq!(
        *app.world().get::<Visibility>(plane).unwrap(),
        Visibility::Hidden
    );
    assert!(
        app.world()
            .get::<Transform>(marker)
            .unwrap()
            .translation
            .is_finite()
    );
    app.world_mut().resource_mut::<AppliedEquation>().equations = default_equations();
    tick(&mut app, 0.0);
    assert_eq!(
        *app.world().get::<Visibility>(marker).unwrap(),
        Visibility::Visible
    );
    assert_eq!(
        *app.world().get::<Visibility>(plane).unwrap(),
        Visibility::Visible
    );
    assert_eq!(
        app.world().get::<Transform>(marker).unwrap().translation,
        Vec3::new(1.0, 2.0, 3.0)
    );
}
