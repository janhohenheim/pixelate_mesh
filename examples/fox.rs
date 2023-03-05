use bevy::prelude::*;
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa { samples: 1 })
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(load_gltf)
        .add_system(setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct ToSpawn {
    fox: Handle<Scene>,
}

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("Fox.glb#Scene0");
    commands.insert_resource(ToSpawn { fox: handle });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    to_spawn: Option<Res<ToSpawn>>,
) {
    let fox_handle = match to_spawn {
        Some(to_spawn) => to_spawn.fox.clone(),
        None => {
            return;
        }
    };
    commands.spawn((
        Name::new("Fox"),
        Pixelate {
            horizontal_pixels: 64,
            vertical_pixels: 64,
        },
        SceneBundle {
            scene: fox_handle,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(shape::Plane { size: 500000.0 }.into()),
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

    commands.remove_resource::<ToSpawn>();
}
