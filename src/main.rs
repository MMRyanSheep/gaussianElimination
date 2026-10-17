use bevy::prelude::*;

use bevy::window::WindowResolution;

#[derive(Component)]
struct PlaneId(u32);

#[derive(Component)]
struct PlaneName(String);

#[derive(Resource)]
struct Settings {
    show_planes: bool,
    opacity: u32,
}

fn setup(mut commands: Commands) {
    for id in 1..=3 {
        commands.spawn((PlaneId(id), PlaneName(id.to_string())));
    }
}

fn print_plane(query: Query<(&PlaneId, &PlaneName)>, res: Res<Settings>) {
    println!("Three plane entities created!");
    println!("--- Demo Settings ---");
    println!("Show planes: {}", res.show_planes);
    println!("Opacity: {}", res.opacity);
    println!("--- Plane Entities ---");
    for plane in &query {
        println!(
            "Plane {} | visible: {} | opacity: {}",
            plane.1.0, res.show_planes, res.opacity
        );
    }
}

// fn print_settings(res: Res<Settings>) {
//     for reso in &res {
//         println!("{}", reso);
//     }
// }

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
        .insert_resource(Settings {
            show_planes: true,
            opacity: 128,
        })
        .add_systems(Startup, (setup, print_plane).chain())
        .run();
}
