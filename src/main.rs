mod state;
mod visualization;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<state::EquationSet>()
        .add_systems(Startup, visualization::setup_scene)
        .run();
}
