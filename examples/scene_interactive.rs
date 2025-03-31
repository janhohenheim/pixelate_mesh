use bevy::{input::mouse::MouseMotion, prelude::*};
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelateMeshPlugin::<MainCamera>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
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
        Transform::from_xyz(0.0, 150.0, 300.0),
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

fn move_camera(
    time: Res<Time>,
    mut camera_transform: Single<&mut Transform, With<MainCamera>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let dt = time.delta_secs();
    let total_motion: Vec2 = mouse_motion_events.read().map(|e| e.delta).sum();
    let sensitivity = 0.1;
    let motion = total_motion * sensitivity * dt;
    let pitch = Quat::from_axis_angle(*camera_transform.right(), -motion.y);
    let yaw = Quat::from_rotation_y(-motion.x);
    let target = Vec3::Y * 50.0;
    camera_transform.rotate_around(target, pitch * yaw);
    camera_transform.look_at(target, Vec3::Y);
}
