use bevy::prelude::*;
use bevy::window::WindowResolution;

#[derive(Component)]
struct PlaneId(u8);

fn setup(mut commands: Commands) {
    for id in 1..=3 {
        let entity = commands.spawn(PlaneId(id)).id();
        println!("Created entity {}", entity)
    }
    

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Quadratic Formula".into(),
                resolution: WindowResolution::new(1000, 700),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
