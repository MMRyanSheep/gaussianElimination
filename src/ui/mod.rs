use crate::state::LinearEquation;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::state::{AppliedEquation, DraftEquation, default_equations, equations_to_srting};

pub fn equation_ui_system(
    mut contexts: EguiContexts,
    mut draft: ResMut<DraftEquation>,
    mut applied: ResMut<AppliedEquation>,
) -> Result {
    egui::Window::new("Equation Input")
        .default_width(420.0)
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
                            applied.equations = equations;
                            draft.error = None;
                        }
                        Err(message) => {
                            draft.error = Some(message);
                        }
                    }
                }

                if ui.button("Reset").clicked() {
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
