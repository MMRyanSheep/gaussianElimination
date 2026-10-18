use crate::state::{Settings, PlaneId};

use bevy::prelude::*;

pub fn spawn_planes(mut commands: Commands) {
    for i in 1..=3 {
            commands.spawn(PlaneId(i));
    }
}

pub fn print_planes(res: Res<Settings>, plane_id: Query<&PlaneId>) {
    println!("setting: {}", res.show_labels);
    for query in &plane_id {
        println!("planeid: {}", query.0)
    }
}
