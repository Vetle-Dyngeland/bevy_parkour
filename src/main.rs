use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_rapier3d::prelude::*;

mod exit;
mod player;
mod scene;

struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(scene::ScenePlugin)
            .add(player::PlayerPlugin)
    }
}

struct OtherPlugins;

impl PluginGroup for OtherPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(exit::ExitPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::default())
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(10, 10, 10)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy parkour game".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::BorderlessFullscreen,
                resolution: (1920f32, 1080f32).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugins)
        .add_plugins(OtherPlugins)
        .run();
}
