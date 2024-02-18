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
            mesh: meshes.add(Plane3d::default()),
            material: materials.add(StandardMaterial::from(Color::WHITE)),
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

fn move_pixelated(time: Res<Time>, mut pixelated: Query<&mut Transform, With<Pixelate>>) {
    for mut transform in pixelated.iter_mut() {
        let (sin, cos) = time.elapsed_seconds().sin_cos();
        let sin = sin * sin * sin;
        let cos = cos * cos * cos;
        transform.translation.x += sin;
        transform.translation.z += cos;
        transform.translation.y = sin.abs() * 20.0;
    }
}
