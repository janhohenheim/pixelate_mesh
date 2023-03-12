use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "bevy=info,wgpu=error,naga=warn,pixelate_mesh=debug".to_string(),
                }),
        )
        .insert_resource(Msaa::Off)
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(setup)
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
        Pixelate {
            horizontal_pixels: 64,
            vertical_pixels: 64,
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
            ..default()
        },
    ));
}
