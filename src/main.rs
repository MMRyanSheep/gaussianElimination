mod visualization;
mod state;

use visualization::{print_planes, spawn_planes};
use state::Settings;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Settings>()
        .add_systems(Startup,(spawn_planes, print_planes).chain())
        .run();
}

