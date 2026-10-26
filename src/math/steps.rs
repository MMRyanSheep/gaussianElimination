use crate::math::carmer::{CramerSolution, replace_column};
use crate::state::LinearEquation;

pub type Matrix3 = [[f64; 3]; 3];

#[derive(Clone, Debug)]
pub enum CalculationStep {
    ShowMatrix {
        matrix: Matrix3,
        rhs: [f64; 3],
    },

    Determinant {
        name: &'static str,
        matrix: Matrix3,
        replaced_col: Option<usize>,
        products: [f64; 6],
        result: f64,
    },

    Solution {
        x: f64,
        y: f64,
        z: f64,
    },
}

fn equation_matrix(equations: &[LinearEquation; 3]) -> (Matrix3, [f64; 3]) {
    let matrix = [
        [
            equations[0].a as f64,
            equations[0].b as f64,
            equations[0].c as f64,
        ],
        [
            equations[1].a as f64,
            equations[1].b as f64,
            equations[1].c as f64,
        ],
        [
            equations[2].a as f64,
            equations[2].b as f64,
            equations[2].c as f64,
        ],
    ];

    let rhs = [
        equations[0].d as f64,
        equations[1].d as f64,
        equations[2].d as f64,
    ];

    return (matrix, rhs);
}

fn determinant_products(m: Matrix3) -> [f64; 6] {
    let a = m[0][0];
    let b = m[0][1];
    let c = m[0][2];

    let d = m[1][0];
    let e = m[1][1];
    let f = m[1][2];

    let g = m[2][0];
    let h = m[2][1];
    let i = m[2][2];

    return [
        a * e * i,
        b * f * g,
        c * d * h,
        -c * e * g,
        -b * d * i,
        -a * f * h,
    ];
}

pub fn generate_steps(
    equations: &[LinearEquation; 3],
    solution: &CramerSolution,
) -> Vec<CalculationStep> {
    let (matrix, rhs) = equation_matrix(equations);

    let _ax = replace_column(matrix, rhs, 0);
    let _ay = replace_column(matrix, rhs, 1);
    let _az = replace_column(matrix, rhs, 2);
    let mut steps = Vec::with_capacity(6);

    steps.push(CalculationStep::ShowMatrix { matrix, rhs });

    steps.push(CalculationStep::Determinant {
        name: "D",
        matrix,
        replaced_col: None,
        products: determinant_products(matrix),
        result: solution.d,
    });

    steps.push(CalculationStep::Determinant {
        name: "Dx",
        matrix,
        replaced_col: Some(0),
        products: determinant_products(matrix),
        result: solution.d,
    });
    steps.push(CalculationStep::Determinant {
        name: "Dy",
        matrix,
        replaced_col: Some(1),
        products: determinant_products(matrix),
        result: solution.d,
    });

    steps.push(CalculationStep::Determinant {
        name: "Dz",
        matrix,
        replaced_col: Some(2),
        products: determinant_products(matrix),
        result: solution.d,
    });

    steps.push(CalculationStep::Solution {
        x: solution.x,
        y: solution.y,
        z: solution.z,
    });

    return steps;
}
