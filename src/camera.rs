use bevy::{input::mouse::MouseMotion, prelude::*};

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init)
            .add_system(camera_rotation);
    }
}

#[derive(Component)]
pub struct MouseRotation {
    pub enabled: bool,
    pub sensitivity: Vec2,
    pub y_clamp: f32,
    pitch: f32,
    yaw: f32
}

impl MouseRotation {
    pub fn new(enabled: bool, sensitivity: Vec2, y_clamp: f32) -> Self {
        Self {
            enabled,
            sensitivity,
            y_clamp,
            pitch: 0f32,
            yaw: 0f32
        }
    }
}

fn init(mut cmd: Commands) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0f32, 5f32, 0f32),
            ..Default::default()
        },
        MouseRotation::new(true, (25f32, 25f32).into(), 90f32),
        Name::from("Camera")
    ));
}

fn camera_rotation(
    mut query: Query<(&mut MouseRotation, &mut Transform), With<Camera>>,
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_events.iter() {
        delta += ev.delta;
    }

    if delta.is_nan() || delta == Vec2::ZERO || !mouse.pressed(MouseButton::Right) {
        return;
    }

    for (mut cam, mut transform) in query.iter_mut() {
        if !cam.enabled {
            continue;
        }

        cam.yaw -= delta.x * cam.sensitivity.x * time.delta_seconds();
        cam.pitch += delta.y * cam.sensitivity.y * time.delta_seconds();

        let clamp = cam.y_clamp - 0.01f32 * (cam.y_clamp > 0f32) as i8 as f32;

        cam.pitch = cam.pitch.clamp(-clamp, clamp);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians())
            * Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
    }
}
