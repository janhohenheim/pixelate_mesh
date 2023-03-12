use bevy::{input::mouse::MouseMotion, prelude::*};
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(setup)
        .add_system(move_camera)
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
        Name::new("Fox"),
        Pixelate::splat(128),
        SceneBundle {
            scene: asset_server.load("Fox.glb#Scene0"),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(100.0, 100.0, 150.0)
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

fn move_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let dt = time.delta_seconds();
    let mut camera_transform = query.single_mut();
    let total_motion: Vec2 = mouse_motion_events.iter().map(|e| e.delta).sum();
    let sensitivity = 0.1;
    let motion = total_motion * sensitivity * dt;
    let pitch = Quat::from_axis_angle(camera_transform.right(), -motion.y);
    let yaw = Quat::from_rotation_y(-motion.x);
    camera_transform.rotate_around(Vec3::ZERO, pitch * yaw);
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}
