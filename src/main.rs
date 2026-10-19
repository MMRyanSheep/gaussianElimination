mod state;
mod visualization;

use bevy::prelude::*;
use state::Settings;
use visualization::scene::init_scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Settings>()
        .add_systems(Startup, init_scene)
        .run();
}
