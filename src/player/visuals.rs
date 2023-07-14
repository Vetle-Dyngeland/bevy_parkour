use bevy::prelude::*;

use super::{Player, PlayerStartupSet};

pub(super) struct PlayerVisualsPlugin;

impl Plugin for PlayerVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init.in_set(PlayerStartupSet::Visuals));
    }
}

fn init(
    mut cmd: Commands,
    player_query: Query<Entity, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.entity(player_query.single()).insert((
        PbrBundle {
            mesh: meshes.add(
                shape::Capsule {
                    radius: 0.5f32,
                    depth: 1f32,
                    ..Default::default()
                }
                .into(),
            ),
            material: materials.add(Color::rgb_u8(150, 150, 255).into()),
            transform: Transform::from_xyz(0f32, 2f32, 0f32),
            ..Default::default()
        },
        PointLight {
            range: 10f32,
            intensity: 1500f32,
            shadows_enabled: false,
            ..Default::default()
        },
    ));
}
