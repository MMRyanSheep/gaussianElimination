use bevy::prelude::*;
#[derive(Component)]
pub struct PlaneId(pub usize);

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
