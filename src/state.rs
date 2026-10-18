
use bevy::prelude::*;
#[derive(Component)]
pub struct PlaneId(pub u8);

#[derive(Resource)]
pub struct Settings {
    pub show_labels: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_labels: true,
        }
    }
}
