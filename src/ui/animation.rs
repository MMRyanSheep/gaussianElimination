use bevy_egui::egui::{self, Color32, RichText};

use crate::{math::steps::CalculationStep, state::StepState};

// Indices of the six signed Sarrus products, in the same order as step data.
const TERMS: [[(usize, usize); 3]; 6] = [
    [(0, 0), (1, 1), (2, 2)],
    [(0, 1), (1, 2), (2, 0)],
    [(0, 2), (1, 0), (2, 1)],
    [(0, 2), (1, 1), (2, 0)],
    [(0, 1), (1, 0), (2, 2)],
    [(0, 0), (1, 2), (2, 1)],
];

pub fn show_animation(ui: &mut egui::Ui, steps: &mut StepState) {
    ui.label("Cramer's rule");
    ui.add_enabled_ui(!steps.steps.is_empty(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                steps.restart();
            }
            if ui
                .add_enabled(steps.current > 0, egui::Button::new("Previous"))
                .clicked()
            {
                steps.select(steps.current - 1);
            }
            if ui
                .add_enabled(
                    steps.current + 1 < steps.steps.len(),
                    egui::Button::new("Next"),
                )
                .clicked()
            {
                steps.select(steps.current + 1);
            }
            if ui
                .button(if steps.playing { "Pause" } else { "Play" })
                .clicked()
            {
                if !steps.playing
                    && steps.current + 1 == steps.steps.len()
                    && steps.elapsed >= steps.duration()
                {
                    steps.restart();
                }
                steps.playing = !steps.playing;
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Slower").clicked() {
                steps.speed = (steps.speed / 2.0).max(0.25);
            }
            ui.label(format!("{:.2}x", steps.speed));
            if ui.button("Faster").clicked() {
                steps.speed = (steps.speed * 2.0).min(4.0);
            }
        });
    });
    let Some(step) = steps.steps.get(steps.current) else {
        return;
    };
    ui.label(format!(
        "Step {} / {}",
        steps.current + 1,
        steps.steps.len()
    ));
    let active = steps.active_product();
    // Time-driven emphasis fades within each product; pausing freezes it.
    let pulse = 0.65 + 0.35 * (steps.elapsed.fract() * std::f32::consts::PI).sin();
    let highlight = Color32::from_rgb((255.0 * pulse) as u8, (215.0 * pulse) as u8, 55);
    match step {
        CalculationStep::ShowMatrix { matrix, rhs } => {
            ui.label("A · [x, y, z] = r");
            show_matrix(ui, matrix, Some(rhs), None, None, highlight);
        }
        CalculationStep::Determinant {
            name,
            matrix,
            replaced_col,
            products,
            result,
        } => {
            if let Some(col) = replaced_col {
                ui.label(format!(
                    "A{}: replace column {} with r",
                    ["x", "y", "z"][*col],
                    col + 1
                ));
            } else {
                ui.label("D = det(A)");
            }
            show_matrix(ui, matrix, None, *replaced_col, Some(active), highlight);
            ui.label("det = aei + bfg + cdh - ceg - bdi - afh");
            for (index, cells) in TERMS.iter().enumerate() {
                let [a, b, c] = cells.map(|(row, col)| matrix[row][col]);
                let text = format!(
                    "{} ({a:.4}) × ({b:.4}) × ({c:.4}) = {:.4}",
                    if index < 3 { "+" } else { "−" },
                    products[index]
                );
                ui.label(if index == active {
                    RichText::new(text).color(highlight).strong()
                } else {
                    RichText::new(text)
                });
            }
            let partial: f64 = products[..=active].iter().sum();
            ui.label(format!("Running sum ({}/6) = {partial:.6}", active + 1));
            ui.label(format!("{name} = {result:.6}"));
        }
        CalculationStep::Solution { x, y, z } => {
            for (axis, numerator, value) in [("x", "Dx", x), ("y", "Dy", y), ("z", "Dz", z)] {
                let denominator = determinant(steps, "D");
                let top = determinant(steps, numerator);
                ui.label(format!(
                    "{axis} = {numerator}/D = {top:.6}/{denominator:.6} = {value:.6}"
                ));
            }
            ui.label(format!("Intersection: ({x:.6}, {y:.6}, {z:.6})"));
        }
    }
    ui.label("Planes: Eq 1 magenta · Eq 2 blue · Eq 3 green");
    ui.label("Gold point: intersection");
    if steps.playing {
        ui.ctx().request_repaint();
    }
}

fn determinant(steps: &StepState, target: &str) -> f64 {
    steps
        .steps
        .iter()
        .find_map(|step| match step {
            CalculationStep::Determinant { name, result, .. } if *name == target => Some(*result),
            _ => None,
        })
        .unwrap_or(0.0)
}

fn show_matrix(
    ui: &mut egui::Ui,
    matrix: &[[f64; 3]; 3],
    rhs: Option<&[f64; 3]>,
    replaced: Option<usize>,
    active: Option<usize>,
    highlight: Color32,
) {
    egui::Grid::new("calculation_matrix").show(ui, |ui| {
        for row in 0..3 {
            ui.label("[");
            for col in 0..3 {
                let mut text = RichText::new(format!("{:>10.4}", matrix[row][col])).monospace();
                if replaced == Some(col) {
                    text = text.background_color(Color32::from_rgb(45, 65, 95));
                }
                if active.is_some_and(|term| TERMS[term].contains(&(row, col))) {
                    text = text.color(highlight).strong();
                }
                ui.label(text);
            }
            ui.label("]");
            if let Some(rhs) = rhs {
                ui.label(format!("   {:>10.4}", rhs[row]));
            }
            ui.end_row();
        }
    });
}
