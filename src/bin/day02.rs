fn solver(equation: &LinearEquation, x: f64, y: f64, z: f64) -> f64 {
    x * equation.a + y * equation.b + z * equation.c - equation.d
}
fn main() {
    let eq1 = LinearEquation {
        a: 1.0,
        b: 1.0,
        c: 1.0,
        d: 6.0,
    };
    let result: String = solver(&eq1, 1.0, 2.0, 3.0).to_string();
    println!("{}", result);
}

struct LinearEquation {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}
