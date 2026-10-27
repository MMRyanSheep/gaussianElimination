use bevy::prelude::*;

use crate::math::{
    carmer::{CramerSolution, solver},
    determinant::equation_to_matrix,
    steps::{CalculationStep, generate_steps},
};
#[derive(Component)]
pub struct PlaneId(pub usize);

#[derive(Resource, Default)]
pub struct CalculationState {
    pub result: Option<CramerSolution>,
}

#[derive(Resource)]
pub struct StepState {
    pub steps: Vec<CalculationStep>,
    pub current: usize,
    pub playing: bool,
    pub elapsed: f32,
    pub speed: f32,
}

impl Default for StepState {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            current: 0,
            playing: false,
            elapsed: 0.0,
            speed: 1.0,
        }
    }
}

impl StepState {
    pub fn restart(&mut self) {
        self.current = 0;
        self.playing = false;
        self.elapsed = 0.0;
    }

    pub fn select(&mut self, index: usize) {
        if self.steps.is_empty() {
            return;
        }
        self.current = index.min(self.steps.len() - 1);
        self.elapsed = 0.0;
        self.playing = false;
    }

    pub fn duration(&self) -> f32 {
        match self.steps.get(self.current) {
            Some(CalculationStep::Determinant { .. }) => 6.0,
            _ => 2.0,
        }
    }

    pub fn active_product(&self) -> usize {
        (self.elapsed.floor() as usize).min(5)
    }
}

pub fn advance_animation(time: Res<Time>, mut steps: ResMut<StepState>) {
    if !steps.playing || steps.steps.is_empty() {
        return;
    }
    steps.elapsed += time.delta_secs() * steps.speed;
    while steps.elapsed >= steps.duration() {
        let duration = steps.duration();
        if steps.current + 1 == steps.steps.len() {
            steps.elapsed = duration;
            steps.playing = false;
            break;
        }
        steps.elapsed -= duration;
        steps.current += 1;
    }
}
#[derive(Debug, Component, Clone, Copy)]
pub struct LinearEquation {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[derive(Resource)]
pub struct DraftEquation {
    pub values: [[String; 4]; 3],
    pub error: Option<String>,
}

#[derive(Resource)]
pub struct AppliedEquation {
    pub equations: [LinearEquation; 3],
}
pub fn default_equations() -> [LinearEquation; 3] {
    [
        LinearEquation {
            a: 1.0,
            b: 1.0,
            c: 1.0,
            d: 6.0,
        },
        LinearEquation {
            a: 2.0,
            b: -1.0,
            c: 1.0,
            d: 3.0,
        },
        LinearEquation {
            a: 1.0,
            b: 2.0,
            c: -1.0,
            d: 2.0,
        },
    ]
}

pub fn equations_to_srting(eq: &[LinearEquation; 3]) -> [[String; 4]; 3] {
    std::array::from_fn(|i| {
        let e = &eq[i];
        [
            e.a.to_string(),
            e.b.to_string(),
            e.c.to_string(),
            e.d.to_string(),
        ]
    })
}

impl Default for AppliedEquation {
    fn default() -> Self {
        Self {
            equations: default_equations(),
        }
    }
}

impl Default for DraftEquation {
    fn default() -> Self {
        Self {
            values: equations_to_srting(&default_equations()),
            error: None,
        }
    }
}

pub fn update_result(
    applied: Res<AppliedEquation>,
    mut calculation: ResMut<CalculationState>,
    mut steps: ResMut<StepState>,
) {
    if !applied.is_changed() {
        return;
    }
    calculation.result = None;
    steps.restart();
    let matrix = equation_to_matrix(&applied);
    let rhs = [
        applied.equations[0].d as f64,
        applied.equations[1].d as f64,
        applied.equations[2].d as f64,
    ];
    let result = solver(matrix, rhs);
    steps.steps = if result.can_animate() {
        generate_steps(&applied.equations, &result)
    } else {
        Vec::new()
    };
    calculation.result = Some(result);
}
