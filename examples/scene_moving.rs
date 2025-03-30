use bevy::prelude::*;
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelateMeshPlugin::<MainCamera>::default())
        .add_systems(Update, move_pixelated)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Fox"),
        Pixelate::splat(128),
        SceneRoot(asset_server.load("Fox.glb#Scene0")),
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

fn move_pixelated(time: Res<Time>, mut pixelated: Query<&mut Transform, With<Pixelate>>) {
    for mut transform in pixelated.iter_mut() {
        let (sin, cos) = time.elapsed_secs().sin_cos();
        let sin = sin * sin * sin;
        let cos = cos * cos * cos;
        transform.translation.x = sin * 80.0;
        transform.translation.z = cos * 80.0;
        transform.translation.y = sin.abs() * 10.0;
        transform.look_at(Vec3::new(0.0, 0.0, -200.0), Vec3::Y);
    }
}
