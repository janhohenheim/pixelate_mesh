use crate::util::get_max_radius;
use crate::{Canvas, Pixelate, PixelationCamera};
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
    utils::{HashMap, HashSet},
};
use std::f32::consts::PI;
use std::iter;

#[derive(Debug, Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct ToPixelate(HashSet<Entity>);

pub(crate) fn mark_for_pixelation(
    mut pixelate_query: Query<Entity, Added<Pixelate>>,
    mut to_pixelate: ResMut<ToPixelate>,
) {
    for entity in &mut pixelate_query {
        to_pixelate.0.insert(entity);
    }
}

pub(crate) fn add_pixelation(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    pixelate_query: Query<(&Pixelate, Option<&Handle<Mesh>>)>,
    mesh_handles: Query<&Handle<Mesh>>,
    children: Query<&Children>,
    mut to_pixelate: ResMut<ToPixelate>,
) {
    let mut ready = HashMap::new();
    for entity in to_pixelate.iter().copied() {
        let self_and_descendants = iter::once(entity).chain(children.iter_descendants(entity));
        for entity in self_and_descendants {
            commands.entity(entity).insert(RenderLayers::layer(1));
            if let Ok(mesh_handle) = mesh_handles.get(entity) {
                if meshes.contains(mesh_handle) {
                    ready.insert(entity, mesh_handle);
                }
            }
        }
    }
    to_pixelate.0 = to_pixelate
        .difference(&ready.keys().copied().collect())
        .copied()
        .collect();
    for (entity, mesh_handle) in ready.drain() {
        let (pixelate, _mesh_handle) = pixelate_query.get(entity).unwrap();
        // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
        let first_pass_layer = RenderLayers::layer(1);

        let image = create_image(*pixelate);
        let image_handle = images.add(image);
        let mesh = meshes.get_mut(mesh_handle).unwrap();
        let aabb = mesh.compute_aabb().unwrap();
        let plane_handle = meshes.add(create_canvas_mesh(&aabb));
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

        // Main pass cube, with material containing the rendered first pass texture.
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
    }
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
