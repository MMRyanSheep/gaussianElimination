use bevy::prelude::*;

use super::scene::{ActiveScene, SceneContent};

pub const MIN_SCALE: f32 = 0.5;
pub const MAX_SCALE: f32 = 2.5;

#[derive(Component)]
pub struct ScalingCube;

#[derive(Resource)]
pub struct CubeAnimation {
    pub playing: bool,
    pub speed: f32,
    pub scale: f32,
    phase: f32,
}

impl Default for CubeAnimation {
    fn default() -> Self {
        Self {
            playing: true,
            speed: 1.0,
            scale: 1.5,
            phase: 0.0,
        }
    }
}

impl CubeAnimation {
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(MIN_SCALE, MAX_SCALE);
        self.phase = (self.scale - 1.5).asin();
        self.playing = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn animation_grows_and_shrinks_within_bounds() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(ActiveScene::Cube)
            .init_resource::<CubeAnimation>()
            .add_systems(Update, animate_cube);
        let cube = app
            .world_mut()
            .spawn((ScalingCube, Transform::default()))
            .id();
        let mut smallest = MAX_SCALE;
        let mut largest = MIN_SCALE;
        for _ in 0..240 {
            app.world_mut()
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f32(1.0 / 30.0));
            app.update();
            let scale = app.world().get::<Transform>(cube).unwrap().scale;
            assert_eq!(scale.x, scale.y);
            assert_eq!(scale.y, scale.z);
            assert!((MIN_SCALE..=MAX_SCALE).contains(&scale.x));
            smallest = smallest.min(scale.x);
            largest = largest.max(scale.x);
        }
        assert!(smallest < MIN_SCALE + 0.01);
        assert!(largest > MAX_SCALE - 0.01);

        app.world_mut()
            .resource_mut::<CubeAnimation>()
            .set_scale(2.0);
        app.update();
        assert_eq!(
            app.world().get::<Transform>(cube).unwrap().scale,
            Vec3::splat(2.0)
        );
        app.world_mut().resource_mut::<CubeAnimation>().playing = true;
        *app.world_mut().resource_mut::<ActiveScene>() = ActiveScene::Plane;
        app.update();
        assert_eq!(app.world().resource::<CubeAnimation>().scale, 2.0);
    }
}

pub fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let border = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.8, 1.0),
        unlit: true,
        ..default()
    });
    commands
        .spawn((
            Name::new("Scaling cube outline"),
            ScalingCube,
            SceneContent(ActiveScene::Cube),
            Transform::from_scale(Vec3::splat(CubeAnimation::default().scale)),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Four edges per direction give twelve edges without face diagonals.
            for axis in 0..3 {
                let mut dimensions = Vec3::splat(0.018);
                dimensions[axis] = 1.0;
                let edge = meshes.add(Cuboid::from_size(dimensions));
                for a in [-0.5, 0.5] {
                    for b in [-0.5, 0.5] {
                        let mut position = Vec3::ZERO;
                        position[(axis + 1) % 3] = a;
                        position[(axis + 2) % 3] = b;
                        parent.spawn((
                            Mesh3d(edge.clone()),
                            MeshMaterial3d(border.clone()),
                            Transform::from_translation(position),
                        ));
                    }
                }
            }
        });
}

pub fn animate_cube(
    time: Res<Time>,
    active: Res<ActiveScene>,
    mut animation: ResMut<CubeAnimation>,
    mut cubes: Query<&mut Transform, With<ScalingCube>>,
) {
    if *active != ActiveScene::Cube {
        return;
    }
    if animation.playing {
        animation.phase =
            (animation.phase + time.delta_secs() * animation.speed) % std::f32::consts::TAU;
        animation.scale = 1.5 + animation.phase.sin();
    }
    for mut transform in &mut cubes {
        transform.scale = Vec3::splat(animation.scale);
    }
}
