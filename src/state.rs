
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
    pub message: Option<String>,
}

#[derive(Resource, Default)]
pub struct StepState {
    pub steps: Vec<CalculationStep>,
    pub current: usize,
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
#[derive(Resource)]
pub struct EquationSet {
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
impl Default for EquationSet {
    fn default() -> Self {
        Self {
            equations: [
                LinearEquation {
                    a: 1.0,
                    b: 0.0,
                    c: 0.0,
                    d: 1.0,
                },
                LinearEquation {
                    a: 0.0,
                    b: 1.0,
                    c: 0.0,
                    d: 2.0,
                },
                LinearEquation {
                    a: 0.0,
                    b: 0.0,
                    c: 1.0,
                    d: 3.0,
                },
            ],
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
    calculation.message = None;
    steps.current = 0;
    let matrix = equation_to_matrix(&applied);
    let rhs = [
        applied.equations[0].d as f64,
        applied.equations[1].d as f64,
        applied.equations[2].d as f64,
    ];
    let result = solver(matrix, rhs);
    steps.steps = generate_steps(&applied.equations, &result);
    calculation.result = Some(result);
}
