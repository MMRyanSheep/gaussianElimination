use bevy::input::mouse::{AccumulatedMouseMotion, MouseWheel};
use bevy::prelude::*;

#[derive(Component)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub target: Vec3,
    pub radius: f32,
}

pub fn camera_transform_update(transform: &mut Transform, camera: &OrbitCamera) {
    let x = camera.radius * camera.pitch.cos() * camera.yaw.sin();
    let y = camera.radius * camera.pitch.sin();
    let z = camera.radius * camera.pitch.cos() * camera.yaw.cos();

    let offset = Vec3::new(x, y, z);
    transform.translation = camera.target + offset;

    let _ = transform.looking_at(camera.target, Vec3::Y);
}

pub fn camera_control(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let mut scroll = 0.0;

    for event in mouse_wheel.read() {
        scroll += event.y
    }

    for (mut transform, mut camera) in query {
        if mouse_button.pressed(MouseButton::Right) {
            camera.pitch += mouse_motion.delta.y * 0.005;
            camera.yaw -= mouse_motion.delta.x * 0.005
        }

        camera.pitch = camera.pitch.clamp(-1.5, 1.5);

        camera.radius *= (-scroll * 0.1).exp();
        camera.radius = camera.radius.clamp(1.0, 90.0);
        camera_transform_update(&mut transform, &camera);

    }
}
