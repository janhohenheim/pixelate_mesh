use crate::util::get_max_radius;
use crate::{Canvas, Pixelate, PixelationCamera};
use bevy::utils::HashMap;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        primitives::Aabb,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    scene::SceneInstance,
    utils::HashSet,
};
use std::f32::consts::PI;

#[derive(Debug, Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct ToPixelate(HashSet<Entity>);

pub(crate) fn mark_for_pixelation(
    mut pixelate_query: Query<Entity, Added<Pixelate>>,
    mut to_pixelate: ResMut<ToPixelate>,
) {
    for entity in &mut pixelate_query {
        debug!("Adding entity to pixelation queue");
        to_pixelate.0.insert(entity);
    }
}

pub(crate) fn add_pixelation(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    pixelate_query: Query<&Pixelate>,
    In(pixelation_targets): In<HashMap<Entity, Handle<Mesh>>>,
) {
    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);
    for (&entity, mesh_handle) in pixelation_targets.iter() {
        debug!("Spawning canvas");
        let mesh = meshes.get(mesh_handle).unwrap();
        let aabb = mesh.compute_aabb().unwrap();
        let plane_handle = meshes.add(create_canvas_mesh(&aabb));
        let pixelate = pixelate_query.get(entity).unwrap();
        let image = create_image(*pixelate);
        let image_handle = images.add(image);
        commands.entity(entity).insert(first_pass_layer);
        commands.spawn((
            Name::new("Pixelation Camera"),
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                    ..default()
                },
                camera: Camera {
                    // render before the "main pass" camera
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                ..default()
            },
            PixelationCamera { target: entity },
            first_pass_layer,
        ));

        commands
            .spawn((
                Name::new("Canvas"),
                Canvas { target: entity },
                SpatialBundle::default(),
            ))
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
        /*
        commands
            .entity(entity)
            .insert((first_pass_layer, aabb, image_handle.clone()))
            .with_children(|parent| {
                // The shadow of the cube
                parent.spawn((
                    Name::new("Shadow object"),
                    PbrBundle {
                        mesh: mesh_handle.clone(),
                        material: materials.add(StandardMaterial {
                            base_color: Color::NONE,
                            alpha_mode: AlphaMode::Mask(1.),
                            ..default()
                        }),
                        ..default()
                    },
                ));
            });
         */
    }
}

pub(crate) fn get_ready_pixelation_targets(
    mut to_pixelate: ResMut<ToPixelate>,
    pixelate_query: Query<
        (
            Option<&Handle<Mesh>>,
            Option<&Handle<Scene>>,
            Option<&SceneInstance>,
        ),
        With<Pixelate>,
    >,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    scene_spawner: Res<SceneSpawner>,
) -> HashMap<Entity, Handle<Mesh>> {
    let mut pixelation_targets = HashMap::new();
    for &entity in to_pixelate.iter() {
        let (mesh_handle, scene_handle, scene_instance) = pixelate_query.get(entity).unwrap();
        if scene_handle.is_some() {
            debug!("Pixelating a scene; waiting for it to load...");
            if let Some(scene_instance) = scene_instance {
                debug!("The scene is loaded, waiting for it to be ready...");
                let scene_instance = **scene_instance;
                if scene_spawner.instance_is_ready(scene_instance) {
                    let mesh_handle = scene_spawner
                        .iter_instance_entities(scene_instance)
                        .filter_map(|entity| mesh_handles.get(entity).ok())
                        .next()
                        .unwrap();

                    debug!("The scene is ready!");
                    pixelation_targets.insert(entity, mesh_handle.clone());
                }
            }
        } else if let Some(mesh_handle) = mesh_handle {
            debug!("Pixelating a mesh; waiting for it to load...");
            if meshes.contains(mesh_handle) {
                debug!("The mesh is loaded!");
                pixelation_targets.insert(entity, mesh_handle.clone());
            }
        } else {
            panic!("The Pixelate component can only be added to entities with a Mesh or a Scene, but found neither.");
        }
    }
    let ready = pixelation_targets.keys().copied().collect();
    to_pixelate.0 = to_pixelate.difference(&ready).copied().collect();
    pixelation_targets
}

fn create_canvas_mesh(aabb: &Aabb) -> Mesh {
    let radius = get_max_radius(aabb);
    let size = Vec2::splat(radius * 2.);
    Mesh::from(shape::Quad { size, flip: false })
}

fn create_image(pixelate: Pixelate) -> Image {
    let size = Extent3d {
        width: pixelate.horizontal_pixels,
        height: pixelate.vertical_pixels,
        ..default()
    };
    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("pixelated render to texture"),
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
    image
}
