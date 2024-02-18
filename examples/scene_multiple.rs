use bevy::prelude::*;
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelateMeshPlugin::<MainCamera>::default())
        .add_systems(Update, setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::new("Fox high rez"),
        SceneBundle {
            scene: asset_server.load("Fox.glb#Scene0"),
            transform: Transform::from_xyz(-50.0, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Fox mid rez"),
        Pixelate::splat(256),
        SceneBundle {
            scene: asset_server.load("Fox.glb#Scene0"),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Fox low rez"),
        Pixelate::splat(128),
        SceneBundle {
            scene: asset_server.load("Fox.glb#Scene0"),
            transform: Transform::from_xyz(50.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 120.0, 200.0)
                .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::ZYX,
                0.0,
                1.0,
                -PI / 4.,
            )),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));
}
