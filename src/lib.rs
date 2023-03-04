#![forbid(missing_docs)]
//! This crate provides a plugin for pixelating meshes.

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
};
use std::f32::consts::PI;

/// The plugin type for this crate.
pub struct PixelateMeshPlugin<C: Component> {
    _camera_type: std::marker::PhantomData<C>,
}

impl<C> Plugin for PixelateMeshPlugin<C>
where
    C: Component,
{
    fn build(&self, app: &mut App) {
        app.register_type::<Pixelate>()
            .add_system(position_canvas::<C>)
            .add_system(sync_cameras::<C>)
            .add_system(add_pixelation)
            .run();
    }
}

/// Marks the entity containing a mesh to be pixelated.
#[derive(Debug, Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct Pixelate {
    /// How many pixels wide the final pixelated image should be.
    pub horizontal_pixels: u32,
    /// How many pixels tall the final pixelated image should be.
    pub vertical_pixels: u32,
}

// Marks the main pass cube, to which the texture is applied.
#[derive(Debug, Component, Copy, Clone)]
struct Canvas {
    pub(crate) target: Entity,
}

#[derive(Debug, Component, Copy, Clone)]
struct PixelationCamera {
    pub(crate) target: Entity,
}

fn add_pixelation(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    pixelation_query: Query<(Entity, &Pixelate, &Handle<Mesh>), Added<Pixelate>>,
) {
    for (entity, pixelate, mesh_handle) in &pixelation_query {
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
                    priority: -1,
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

fn get_max_radius(aabb: &Aabb) -> f32 {
    aabb.half_extents.length()
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
        },
        ..default()
    };
    // fill image.data with zeroes
    image.resize(size);
    image
}

/// Syncs the pixelation camera to the main camera.
fn sync_cameras<T: Component>(
    mut commands: Commands,
    mut pixelation_camera_query: Query<(&mut Transform, &PixelationCamera), Without<T>>,
    outer_camera_query: Query<&Transform, (With<T>, Without<PixelationCamera>)>,
    main_object_query: Query<(&Transform, &Aabb), (Without<T>, Without<PixelationCamera>)>,
) {
    for (mut pixelation_camera_transform, pixelation_camera) in &mut pixelation_camera_query {
        for outer_camera_transform in outer_camera_query.iter() {
            if let Ok((main_object_transform, aabb)) =
                main_object_query.get(pixelation_camera.target)
            {
                *pixelation_camera_transform = outer_camera_transform.looking_at(
                    main_object_transform.translation,
                    outer_camera_transform.up(),
                );
                pixelation_camera_transform.translation = main_object_transform.translation;
                let back = pixelation_camera_transform.back();
                let radius = get_max_radius(aabb);

                // Chosen by eye, feel free to explain to me why this works :)
                const DISTANCE_FACTOR: f32 = 3.2;
                pixelation_camera_transform.translation += back * radius * DISTANCE_FACTOR;
            } else {
                commands
                    .entity(pixelation_camera.target)
                    .despawn_recursive();
            }
        }
    }
}

/// Rotates the canvas (main pass)
fn position_canvas<T: Component>(
    mut commands: Commands,
    mut canvas_query: Query<(&mut Transform, &Canvas), Without<T>>,
    outer_camera_query: Query<&Transform, (With<T>, Without<Canvas>)>,
    main_object_query: Query<(&Transform, &Aabb), (Without<T>, Without<Canvas>)>,
) {
    for (mut canvas_transform, canvas) in &mut canvas_query {
        if let Ok((main_object_transform, aabb)) = main_object_query.get(canvas.target) {
            for camera_transform in outer_camera_query.iter() {
                *canvas_transform = main_object_transform
                    .looking_at(camera_transform.translation, camera_transform.up());
                let forward = canvas_transform.forward();
                let radius = get_max_radius(aabb);
                canvas_transform.translation += forward * radius;
            }
        } else {
            commands.entity(canvas.target).despawn_recursive();
        }
    }
}
