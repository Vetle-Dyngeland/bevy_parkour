use super::{PlayerSet, PlayerStartupSet};
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerStartupSet::Camera))
            .add_systems(
                Update,
                (camera_rotation, grab_cursor).in_set(PlayerSet::Camera),
            );
    }
}

fn grab_cursor(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut window = window.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

#[derive(Component)]
pub struct MouseRotation {
    pub enabled: bool,
    pub sensitivity: Vec2,
    pub y_clamp: f32,
    pitch: f32,
    yaw: f32,
}

impl MouseRotation {
    pub fn new(enabled: bool, sensitivity: Vec2, y_clamp: f32) -> Self {
        Self {
            enabled,
            sensitivity,
            y_clamp,
            pitch: 0f32,
            yaw: 0f32,
        }
    }
}

fn init(mut cmd: Commands) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0f32, 5f32, 0f32),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 100f32.to_radians(),
                ..Default::default()
            }),
            ..Default::default()
        },
        MouseRotation::new(true, (25f32, 25f32).into(), 90f32),
        Name::from("Camera"),
    ));
}

fn camera_rotation(
    mut query: Query<(&mut MouseRotation, &mut Transform), With<Camera3d>>,
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_events.iter() {
        delta += ev.delta;
    }

    if delta.is_nan() || delta == Vec2::ZERO {
        return;
    }

    for (mut cam, mut transform) in query.iter_mut() {
        if !cam.enabled {
            continue;
        }

        cam.yaw -= delta.x * cam.sensitivity.x * time.delta_seconds();
        cam.pitch += delta.y * cam.sensitivity.y * time.delta_seconds();

        let clamp = cam.y_clamp;
        cam.pitch = cam.pitch.clamp(-clamp, clamp);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians())
            * Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
    }
}
