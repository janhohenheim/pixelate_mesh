use bevy::{input::mouse::MouseMotion, prelude::*};
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
) {
    commands.spawn((
        Name::new("Cube"),
        Pixelate::splat(64),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(10.0).into()),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                shadow_depth_bias: 0.05,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(5.0, 10.0, 10.0)),
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
