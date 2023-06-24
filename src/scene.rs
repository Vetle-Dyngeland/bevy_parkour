use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(super) struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);       
    }
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.brightness = 0.35f32;

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 100f32,
                subdivisions: 0u32,
            })),
            material: materials.add(Color::rgb_u8(125u8, 125u8, 125u8).into()),
            transform: Transform::from_xyz(0f32, 0f32, 0f32),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50f32, 0.1f32, 50f32),
        Name::from("Ground"),
    ));

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 2f32})),
            material: materials.add(Color::rgb_u8(255, 255, 125).into()),
            transform: Transform::from_xyz(1f32, 1f32, 5f32),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1f32, 1f32, 1f32),
        Name::from("Box")
    ));

    cmd.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10000f32,
                ..Default::default()
            },
            transform: Transform::from_xyz(0f32, 5f32, 0f32).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -1f32,
                0.2f32,
                0f32,
            )),
            ..Default::default()
        },
        Name::from("Light"),
    ));
}
