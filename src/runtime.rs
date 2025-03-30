use crate::creation::{create_canvas_image, create_canvas_material};
use crate::util::get_max_radius;
use crate::{Canvas, Pixelate, PixelationCamera};
use bevy::render::camera::RenderTarget;
use bevy::render::view::VisibleEntities;
use bevy::utils::HashSet;
use bevy::{prelude::*, render::primitives::Aabb};
use std::iter;

/// Syncs the pixelation camera to the main camera.
pub(crate) fn sync_cameras<T: Component>(
    mut commands: Commands,
    mut pixelation_camera_query: Query<(Entity, &mut Transform, &PixelationCamera), Without<T>>,
    outer_camera_query: Query<&Transform, (With<T>, Without<PixelationCamera>)>,
    main_object_query: Query<(&Transform, &Aabb), (Without<T>, Without<PixelationCamera>)>,
) {
    for (entity, mut pixelation_camera_transform, pixelation_camera) in &mut pixelation_camera_query
    {
        for outer_camera_transform in outer_camera_query.iter() {
            if let Ok((main_object_transform, aabb)) =
                main_object_query.get(pixelation_camera.target)
            {
                *pixelation_camera_transform = outer_camera_transform.looking_at(
                    main_object_transform.translation,
                    *outer_camera_transform.up(),
                );
                pixelation_camera_transform.translation = main_object_transform.translation;
                let back = pixelation_camera_transform.back();
                let radius = get_max_radius(aabb);

                // Chosen by eye, feel free to explain to me why this works :)
                const DISTANCE_FACTOR: f32 = 3.2;
                pixelation_camera_transform.translation += back * radius * DISTANCE_FACTOR;
            } else {
                debug!("Despawning pixelation camera because it holds an invalid target.");
                commands.entity(entity).despawn_recursive();
                if let Some(entity_commands) = commands.get_entity(pixelation_camera.target) {
                    entity_commands.despawn_recursive();
                }
            }
        }
    }
}

/// Rotates the canvas (main pass)
pub(crate) fn position_canvas<T: Component>(
    mut commands: Commands,
    mut canvas_query: Query<(Entity, &mut Transform, &Canvas), Without<T>>,
    outer_camera_query: Query<&Transform, (With<T>, Without<Canvas>)>,
    main_object_query: Query<(&Transform, &Aabb), (Without<T>, Without<Canvas>)>,
) {
    for (entity, mut canvas_transform, canvas) in &mut canvas_query {
        if let Ok((main_object_transform, aabb)) = main_object_query.get(canvas.target) {
            for camera_transform in outer_camera_query.iter() {
                *canvas_transform = main_object_transform
                    .looking_at(camera_transform.translation, *camera_transform.up());
                let forward = canvas_transform.forward();
                let radius = get_max_radius(aabb);
                canvas_transform.translation += forward * radius;
            }
        } else {
            debug!("Despawning canvas because it holds an invalid target.");
            commands.entity(entity).despawn_recursive();
            if let Some(entity_commands) = commands.get_entity(canvas.target) {
                entity_commands.despawn_recursive();
            }
        }
    }
}

pub(crate) fn despawn_dependent_types(
    mut commands: Commands,
    mut removed_pixelate: RemovedComponents<Pixelate>,
    canvas_query: Query<Entity, With<Canvas>>,
    pixelation_camera_query: Query<Entity, With<PixelationCamera>>,
) {
    for entity in removed_pixelate.read() {
        debug!("Pixelate was removed from an entity; removing canvas and pixelation camera that held it as target.");
        for canvas in canvas_query.iter() {
            if canvas == entity {
                commands.entity(canvas).despawn_recursive();
            }
        }
        for pixelation_camera in pixelation_camera_query.iter() {
            if pixelation_camera == entity {
                commands.entity(pixelation_camera).despawn_recursive();
            }
        }
    }
}

pub(crate) fn update_pixelation(
    mut commands: Commands,
    pixelate_query: Query<(Entity, &Pixelate), Changed<Pixelate>>,
    mut pixelation_camera_query: Query<(&PixelationCamera, &mut Camera)>,
    canvas_query: Query<(&Canvas, &Children)>,
    with_standard_material: Query<Entity, With<MeshMaterial3d<StandardMaterial>>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (entity, pixelate) in pixelate_query.iter() {
        if let Some((_, mut camera)) = pixelation_camera_query
            .iter_mut()
            .find(|(pixelation_camera, _)| pixelation_camera.target == entity)
        {
            if let Some((_, children)) = canvas_query
                .iter()
                .find(|(canvas, _)| canvas.target == entity)
            {
                if let Some(entity) = children
                    .iter()
                    .find(|entity| with_standard_material.contains(**entity))
                {
                    let image_handle = images.add(create_canvas_image(*pixelate));
                    camera.target = RenderTarget::Image(image_handle.clone());
                    let material_handle =
                        standard_materials.add(create_canvas_material(image_handle));
                    commands
                        .entity(*entity)
                        .insert(MeshMaterial3d(material_handle));
                }
            }
        }
    }
}

pub(crate) fn set_visible(
    mut pixelation_camera_query: Query<(&PixelationCamera, &mut VisibleEntities)>,
    children_query: Query<&Children>,
) {
    for (pixelation_camera, mut visible_entities) in pixelation_camera_query.iter_mut() {
        let parent = pixelation_camera.target;
        let descendants = children_query.iter_descendants(parent);
        let allowed: HashSet<_> = iter::once(parent).chain(descendants).collect();

        visible_entities
            .get_mut::<With<Mesh3d>>()
            .retain(|&entity| allowed.contains(&entity));
    }
}
