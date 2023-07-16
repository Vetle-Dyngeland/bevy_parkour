use bevy::{app::PluginGroupBuilder, prelude::*};

pub(super) struct PlayerPlugin;

mod camera;
mod input;
mod movement;
mod state_machine;
mod visuals;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugins, PlayerSystemSetPlugin))
            .add_systems(PreStartup, init.in_set(PlayerStartupSet::Main));
    }
}

#[derive(Component)]
pub struct Player; // Marker component

fn init(mut cmd: Commands) {
    cmd.spawn((SpatialBundle::default(), Player, Name::from("Player")));
}

struct PlayerSystemSetPlugin;

impl Plugin for PlayerSystemSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                PlayerSet::PrePlayer,
                PlayerSet::Main,
                PlayerSet::Input,
                PlayerSet::StateMachine,
                PlayerSet::Movement,
                PlayerSet::Camera,
                PlayerSet::Visuals,
                PlayerSet::PostPlayer,
            )
                .chain(),
        )
        .configure_sets(
            PreStartup,
            (
                PlayerStartupSet::PrePlayer,
                PlayerStartupSet::Main,
                PlayerStartupSet::Input,
                PlayerStartupSet::StateMachine,
                PlayerStartupSet::Movement,
                PlayerStartupSet::Camera,
                PlayerStartupSet::Visuals,
                PlayerStartupSet::PostPlayer,
            ),
        );

        app.add_systems(
            PreStartup,
            (
                apply_deferred
                    .before(PlayerStartupSet::Main)
                    .after(PlayerStartupSet::PrePlayer),
                apply_deferred
                    .before(PlayerStartupSet::Input)
                    .after(PlayerStartupSet::Main),
                apply_deferred
                    .before(PlayerStartupSet::StateMachine)
                    .after(PlayerStartupSet::Input),
                apply_deferred
                    .before(PlayerStartupSet::Movement)
                    .after(PlayerStartupSet::StateMachine),
                apply_deferred
                    .before(PlayerStartupSet::Camera)
                    .after(PlayerStartupSet::Movement),
                apply_deferred
                    .before(PlayerStartupSet::Visuals)
                    .after(PlayerStartupSet::Camera),
                apply_deferred
                    .before(PlayerStartupSet::PostPlayer)
                    .after(PlayerStartupSet::Visuals),
            ),
        );
    }
}

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerSet {
    PrePlayer,
    Main,
    Input,
    StateMachine,
    Movement,
    Camera,
    Visuals,
    PostPlayer,
}

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerStartupSet {
    PrePlayer,
    Main,
    Input,
    StateMachine,
    Movement,
    Camera,
    Visuals,
    PostPlayer,
}

struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(input::PlayerInputPlugin)
            .add(state_machine::PlayerStateMachinePlugin)
            .add(movement::PlayerMovementPlugin)
            .add(camera::CameraPlugin)
            .add(visuals::PlayerVisualsPlugin)
    }
}
