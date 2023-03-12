//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::{
        camera::RenderTarget,
        primitives::Aabb,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(setup)
        .add_startup_system(load_gltf)
        .add_system(cube_rotator_system)
        .add_system(rotator_system)
        .add_system(on_spawn.before(setup))
        .run();
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

#[derive(Resource)]
struct ToSpawn(Handle<Gltf>);

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("Fox.glb");
    commands.insert_resource(ToSpawn(gltf));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    gltfs: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    to_spawn: Option<Res<ToSpawn>>,
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

    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    // The model that will be rendered to the texture.
    commands.spawn((
        SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        },
        FirstPassCube,
        first_pass_layer,
    ));

    let gltf_mesh = gltf_meshes.get(&gltf.meshes[0]).unwrap();
    let mesh = meshes.get(&gltf_mesh.primitives[0].mesh).unwrap();
    let aabb = mesh.compute_aabb().unwrap();
    let plane_handle = meshes.add(create_canvas_mesh(&aabb));

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        first_pass_layer,
    ));

    // Main pass cube, with material containing the rendered first pass texture.
    commands
        .spawn((Name::new("Canvas"), SpatialBundle::default()))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Canvas Mesh"),
                PbrBundle {
                    mesh: plane_handle,
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(image_handle.clone()),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_rotation(Quat::from_rotation_y(PI)),
                    ..default()
                },
            ));
        });

    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.remove_resource::<ToSpawn>();
}

fn create_canvas_mesh(aabb: &Aabb) -> Mesh {
    let radius = get_max_radius(aabb);
    let size = Vec2::splat(radius * 2.);
    Mesh::from(shape::Quad { size, flip: false })
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds());
        transform.rotate_y(0.7 * time.delta_seconds());
    }
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

pub(crate) fn get_max_radius(aabb: &Aabb) -> f32 {
    aabb.half_extents.length()
}
