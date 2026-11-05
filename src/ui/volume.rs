use bevy::prelude::Color;
use bevy_egui::egui::{self, Color32};

use crate::math::carmer::CramerSolution;
use crate::math::determinant::equation_to_matrix;
use crate::state::AppliedEquation;
use crate::visualization::volume::{
    VolumeSceneState, column_vectors, legend_colors, rhs_vector, signed_volume, solution_vector,
    stage_at, volume_at,
};

const POSITIVE: Color32 = Color32::from_rgb(120, 220, 140);
const NEGATIVE: Color32 = Color32::from_rgb(255, 140, 120);

pub fn show_panel(
    ui: &mut egui::Ui,
    state: &mut VolumeSceneState,
    applied: &AppliedEquation,
    result: Option<&CramerSolution>,
) {
    ui.heading("Cramer volume");
    ui.label("V_i / V0 = x_i");
    ui.separator();

    let Some(result) = result else {
        ui.label("Waiting for the equations ...");
        return;
    };

    if !result.can_animate() {
        ui.colored_label(NEGATIVE, "矩阵奇异，无唯一解");
        ui.small("The parallelepipeds stay hidden while det(A) is zero.");
        return;
    }

    ui.horizontal(|ui| {
        ui.label("替换列:");
        for column in 0..3 {
            let selected = state.column == column;
            let label = format!("第 {} 列 a{}", column + 1, column + 1);
            if ui.radio(selected, label).clicked() && !selected {
                state.select_column(column);
            }
        }
    });

    ui.horizontal(|ui| {
        let label = if state.playing { "Pause" } else { "Play" };
        if ui.button(label).clicked() {
            if !state.playing && state.t >= 1.0 {
                state.restart();
            }
            state.playing = !state.playing;
        }
        if ui.button("Reset").clicked() {
            state.restart();
        }
        ui.label(format!("阶段: {}", stage_at(state.t).label()));
    });

    let mut t = state.t;
    if ui
        .add(egui::Slider::new(&mut t, 0.0..=1.0).text("t"))
        .changed()
    {
        state.set_t(t);
    }
    ui.small(if state.t < 0.5 {
        "0 - 0.5: a_i is scaled by x_i"
    } else {
        "0.5 - 1: x_i * a_i is sheared onto b (volume unchanged)"
    });

    ui.separator();

    let columns = column_vectors(equation_to_matrix(applied));
    let target = rhs_vector(applied);
    let x = solution_vector(result);
    let v0 = signed_volume(columns);
    let current = volume_at(columns, target, x, state.column, state.t);
    let at_end = volume_at(columns, target, x, state.column, 1.0);

    let name = ["1", "2", "3"][state.column];
    ui.label(format!("V0 = det(A) = {v0:.6}"));
    ui.label(format!("V(t) = (u × v) · w = {current:.6}"));
    ui.label(format!("V(t) / V0 = {:.6}", current / v0));
    ui.separator();
    ui.label(format!(
        "x{name} = {:.6}   (V{name} / V0 = {:.6})",
        x[state.column],
        at_end / v0
    ));

    let error = (at_end / v0 - x[state.column]).abs();
    let (mark, color) = if error < 1e-6 {
        ("✓", POSITIVE)
    } else {
        ("✗", NEGATIVE)
    };
    ui.colored_label(
        color,
        format!("{mark} V{name} / V0 = x{name}  (误差 {error:.1e})"),
    );

    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Vectors:");
        for (color, label) in legend_colors() {
            ui.colored_label(to_color32(color), label);
        }
    });
    ui.small("Dashed: a_i → x_i·a_i → b. Drag the scene to orbit.");

    if state.playing {
        ui.ctx().request_repaint();
    }
}

fn to_color32(color: Color) -> Color32 {
    let color = color.to_srgba();
    Color32::from_rgb(
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    )
}
