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
                    b: 1.0,
                    c: 1.0,
                    d: 6.0,
                },
                LinearEquation {
                    a: 1.0,
                    b: 4.0,
                    c: 2.0,
                    d: 7.0,
                },
                LinearEquation {
                    a: 5.0,
                    b: 1.0,
                    c: 3.0,
                    d: 4.0,
                },
            ],
        }
    }
}
