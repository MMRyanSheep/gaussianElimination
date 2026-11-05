mod animation;
mod volume;
use crate::state::LinearEquation;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::state::{
    AppliedEquation, CalculationState, DraftEquation, default_equations, equations_to_srting,
};
use crate::visualization::{
    cube::{CubeAnimation, MAX_SCALE, MIN_SCALE},
    scene::ActiveScene,
    volume::VolumeSceneState,
};

pub fn scene_ui_system(
    mut contexts: EguiContexts,
    mut active: ResMut<ActiveScene>,
    mut animation: ResMut<CubeAnimation>,
    mut volume: ResMut<VolumeSceneState>,
    applied: Res<AppliedEquation>,
    calculation: Res<CalculationState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "scene_viewport".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );
    egui::Panel::top("scene_navigation").show(&mut viewport_ui, |ui| {
        ui.horizontal(|ui| {
            ui.strong("Linear3D");
            ui.separator();
            ui.selectable_value(&mut *active, ActiveScene::Plane, "Plane");
            ui.selectable_value(&mut *active, ActiveScene::Cube, "Cube");
            ui.selectable_value(&mut *active, ActiveScene::Volume, "Volume");
        });
    });

    if *active == ActiveScene::Cube {
        egui::Window::new("Cube")
            .default_width(300.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("motion");
                ui.label("A cube.");
                ui.separator();
                ui.horizontal(|ui| {
                    let label = if animation.playing { "Pause" } else { "Play" };
                    if ui.button(label).clicked() {
                        animation.playing = !animation.playing;
                    }
                    if ui.button("Reset").clicked() {
                        *animation = CubeAnimation::default();
                    }
                    ui.label(if animation.playing {
                        "Animating"
                    } else {
                        "Paused"
                    });
                });
                ui.add(egui::Slider::new(&mut animation.speed, 0.2..=3.0).text("Speed"));
                let mut scale = animation.scale;
                if ui
                    .add(egui::Slider::new(&mut scale, MIN_SCALE..=MAX_SCALE).text("Scale"))
                    .changed()
                {
                    animation.set_scale(scale);
                }
                ui.small("Adjust scale to pause and inspect the cube.");
                ui.separator();
                ui.label(format!("Side length: {:.2}", animation.scale));
                ui.label(format!("Volume: {:.2}", animation.scale.powi(3)));
                ui.horizontal(|ui| {
                    ui.label("Axes:");
                    ui.colored_label(egui::Color32::from_rgb(242, 77, 89), "X");
                    ui.colored_label(egui::Color32::from_rgb(89, 217, 128), "Y");
                    ui.colored_label(egui::Color32::from_rgb(89, 128, 255), "Z");
                });
                ui.small("Drag the scene to orbit. Scroll to zoom.");
            });
    }

    if *active == ActiveScene::Volume {
        egui::Window::new("Volume")
            .default_width(330.0)
            .resizable(false)
            .show(ctx, |ui| {
                volume::show_panel(ui, &mut volume, &applied, calculation.result.as_ref());
            });
    }
    Ok(())
}

pub fn equation_ui_system(
    mut contexts: EguiContexts,
    mut draft: ResMut<DraftEquation>,
    mut applied: ResMut<AppliedEquation>,
    mut steps: ResMut<crate::state::StepState>,
) -> Result {
    egui::Window::new("Equation Input")
        .default_width(420.0)
        .vscroll(true)
        .show(contexts.ctx_mut()?, |ui| {
            ui.heading("3 Linear Equations");

            ui.label("ax + by + cz = d");
            ui.separator();

            egui::Grid::new("equations_grid")
                .num_columns(5)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    ui.label("");
                    for name in ["a", "b", "c", "d"] {
                        ui.label(name);
                    }
                    ui.end_row();

                    for i in 0..3 {
                        ui.label(format!("Eq {}", i + 1));

                        for j in 0..4 {
                            ui.add(
                                egui::TextEdit::singleline(&mut draft.values[i][j])
                                    .desired_width(65.0),
                            );
                        }

                        ui.end_row();
                    }
                });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Apply").clicked() {
                    match parse_equation(&draft.values) {
                        Ok(equations) => {
                            steps.steps.clear();
                            steps.restart();
                            applied.equations = equations;
                            draft.error = None;
                        }
                        Err(message) => {
                            draft.error = Some(message);
                        }
                    }
                }

                if ui.button("Reset").clicked() {
                    steps.steps.clear();
                    steps.restart();
                    applied.equations = default_equations();
                    draft.values = equations_to_srting(&applied.equations);
                    draft.error = None;
                }
            });

            if let Some(message) = &draft.error {
                ui.colored_label(egui::Color32::RED, message);
            }

            ui.separator();
            ui.label("Applied equations:");

            for e in &applied.equations {
                ui.label(format!("({})x + ({})y + ({})z = {}", e.a, e.b, e.c, e.d));
            }

            ui.separator();
            animation::show_animation(ui, &mut steps);
        });

    Ok(())
}

fn parse_equation(values: &[[String; 4]; 3]) -> Result<[LinearEquation; 3], String> {
    let mut result = Vec::with_capacity(3);

    for (i, row) in values.iter().enumerate() {
        let mut numbers = [0.0_f64; 4];
        //error determine
        for (j, text) in row.iter().enumerate() {
            if text.trim().is_empty() {
                return Err(format!("Equation {} field {} is empty", i + 1, j + 1));
            }
            let value = text
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("equation {} field {} is invalid", i + 1, j + 1))?;
            if !value.is_finite() {
                return Err("NaN and infinity is not vaild!".into());
            }
            //logic

            numbers[j] = value;
        }
        result.push(LinearEquation {
            a: numbers[0] as f32,
            b: numbers[1] as f32,
            c: numbers[2] as f32,
            d: numbers[3] as f32,
        })
    }
    Ok(result.try_into().unwrap())
}
