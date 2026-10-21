use bevy::prelude::*;
#[derive(Component)]
pub struct PlaneId(pub usize);

#[derive(Component, Clone, Copy)]
pub struct LinearEquation {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[derive(Resource)]
pub struct EquationSet {
    pub equations: [LinearEquation; 3],
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
