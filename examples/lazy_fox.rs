use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_editor_pls::EditorPlugin;
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EditorPlugin)
        .insert_resource(Msaa::Off)
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(load_gltf)
        .add_system(setup)
        .add_system(on_spawn)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct ToSpawn(Handle<Gltf>);

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("Fox.glb");
    commands.insert_resource(ToSpawn(gltf));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    to_spawn: Option<Res<ToSpawn>>,
    gltfs: Res<Assets<Gltf>>,
) {
    let to_spawn = match to_spawn {
        Some(to_spawn) => to_spawn,
        None => {
            return;
        }
    };
    let gltf = match gltfs.get(&to_spawn.0) {
        Some(gltf) => gltf,
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
            scene: gltf.scenes[0].clone(),
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
    commands.remove_resource::<ToSpawn>();
}

fn on_spawn(
    mut commands: Commands,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    my_loaded_gltf: Option<Res<ToSpawn>>,
    q: Query<(Entity, &Handle<Mesh>)>,
) {
    let my_loaded_gltf = match my_loaded_gltf {
        Some(my_loaded_gltf) => my_loaded_gltf,
        None => {
            return;
        }
    };
    if let Some(gltf) = assets_gltf.get(&my_loaded_gltf.0) {
        for mesh in gltf.meshes.iter() {
            let gltf_mesh = assets_gltfmesh.get(&mesh).unwrap();
            for primitive in gltf_mesh.primitives.iter() {
                for (e, h) in q.iter() {
                    if *h == primitive.mesh {
                        commands.entity(e).insert(RenderLayers::layer(1));
                    }
                }
            }
        }
    }
}
