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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Fox high res"),
        SceneRoot(asset_server.load("Fox.glb#Scene0")),
        Transform::from_xyz(-50.0, 0.0, 0.0),
    ));
    commands.spawn((
        Name::new("Fox mid res"),
        Pixelate::splat(256),
        SceneRoot(asset_server.load("Fox.glb#Scene0")),
    ));
    commands.spawn((
        Name::new("Fox low res"),
        Pixelate::splat(128),
        SceneRoot(asset_server.load("Fox.glb#Scene0")),
        Transform::from_xyz(50.0, 0.0, 0.0),
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 120.0, 200.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Name::new("Light"),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        PIXELATION_RENDER_LAYERS.clone(),
    ));
}
