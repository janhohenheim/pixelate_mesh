use bevy::{input::mouse::MouseMotion, prelude::*};
use pixelate_mesh::prelude::*;

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Cube"),
        Pixelate::splat(64),
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(0., 0., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight {
            shadows_enabled: true,
            shadow_depth_bias: 0.05,
            ..default()
        },
        Transform::from_translation(Vec3::new(5.0, 10.0, 10.0)),
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
    camera_transform.rotate_around(Vec3::ZERO, pitch * yaw);
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}
