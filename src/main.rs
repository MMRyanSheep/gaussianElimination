mod state;
mod visualization;

use bevy::prelude::*;
use state::Settings;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Settings>()
        .add_systems(Startup, visualization::setup_scene)
        .run();
}
