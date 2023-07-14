use std::time::Duration;

use bevy::{app::AppExit, prelude::*};

pub struct ExitPlugin {
    pub keys: Vec<KeyCode>,
    pub reset_timer_duration: Duration,
    pub press_count: usize,
}

impl Default for ExitPlugin {
    fn default() -> Self {
        Self {
            keys: vec![KeyCode::ControlLeft, KeyCode::L],
            reset_timer_duration: Duration::from_secs_f32(0.5f32),
            press_count: 1usize,
        }
    }
}

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ExitKeys {
            reset_timer: Timer::new(self.reset_timer_duration, TimerMode::Once),
            keys: self.keys.clone(),
            press_count: self.press_count,
        })
        .add_systems(Update, exit_system);
    }
}

#[derive(Resource)]
struct ExitKeys {
    reset_timer: Timer,
    keys: Vec<KeyCode>,
    press_count: usize,
}

fn exit_system(
    mut exit_keys: ResMut<ExitKeys>,
    mut count: Local<usize>,
    time: Res<Time>,
    mut appexit: EventWriter<AppExit>,
    keyboard: Res<Input<KeyCode>>,
) {
    exit_keys
        .reset_timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if exit_keys.reset_timer.just_finished() {
        exit_keys.reset_timer.reset();
        *count = 0;
    }

    if *count >= exit_keys.press_count {
        appexit.send(AppExit);
    }

    for key in exit_keys.keys.iter() {
        if !keyboard.pressed(key.clone()) {
            return;
        }
    }

    for key in exit_keys.keys.iter() {
        if keyboard.just_pressed(key.clone()) {
            *count += 1;
            return;
        }
    }
}
