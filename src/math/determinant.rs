use crate::state::EquationSet;
use bevy::prelude::*;

pub fn det3(m: [[f64; 3]; 3]) -> f64 {
    // TODO
    let result_a = m[0][0] * m[1][1] * m[2][2];
    let result_b = m[0][1] * m[1][2] * m[2][0];
    let result_c = m[0][2] * m[1][0] * m[2][1];

    let result_d = m[2][0] * m[1][1] * m[0][2];
    let result_e = m[2][1] * m[1][2] * m[0][0];
    let result_f = m[2][2] * m[1][0] * m[0][1];

    return result_a + result_b + result_c - result_d - result_e - result_f;
}

pub fn equation_to_matrix(eq: &EquationSet) -> [[f64; 3]; 3] {
    return [
        [
            eq.equations[0].a as f64,
            eq.equations[0].b as f64,
            eq.equations[0].c as f64,
        ],
        [
            eq.equations[1].a as f64,
            eq.equations[1].b as f64,
            eq.equations[1].c as f64,
        ],
        [
            eq.equations[2].a as f64,
            eq.equations[2].b as f64,
            eq.equations[2].c as f64,
        ],
    ];
}

pub fn test(eq: Res<EquationSet>) {
    let mat = equation_to_matrix(&eq);
    let d = det3(mat);

    println!("determinant {}", d);
}
