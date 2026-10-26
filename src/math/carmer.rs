use crate::math::determinant::det3;

#[derive(Debug, Clone, Copy)]
pub struct CramerSolution {
    pub d: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn replace_column(mut matrix: [[f64; 3]; 3], rhs: [f64; 3], col: usize) -> [[f64; 3]; 3] {
    for row in 0..3 {
        matrix[row][col] = rhs[row];
    }
    return matrix;
}

pub fn solver(matrix: [[f64; 3]; 3], rhs: [f64; 3], d: f64) -> CramerSolution {
    let dx = det3(replace_column(matrix, rhs, 0));
    let dy = det3(replace_column(matrix, rhs, 1));
    let dz = det3(replace_column(matrix, rhs, 2));

    let x = dx / d;
    let y = dy / d;
    let z = dz / d;

    return CramerSolution {
        d,
        dx,
        dy,
        dz,
        x,
        y,
        z,
    };
}

pub fn max_residual(mat: [[f64; 3]; 3], rhs: [f64; 3], xyz: [f64; 3]) -> f64 {
    let mut max_error = 0.0_f64;

    for row in 1..3 {
        let lhs = mat[row][0] * xyz[0] + mat[row][1] * xyz[1] + mat[row][2] * xyz[2];

        let error = (lhs - rhs[row]).abs();
        max_error = max_error.max(error);
    }
    return max_error;
}
