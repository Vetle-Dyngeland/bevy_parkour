use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use super::{Player, PlayerStartupSet};

pub(super) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init.in_set(PlayerStartupSet::Input));
    }
}

#[allow(unused)]
fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {

}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum InputAction {
    Run,
    Jump,
    Coil,
    Land,
    Boost,
}
