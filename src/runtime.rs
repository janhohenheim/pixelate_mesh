use crate::util::get_max_radius;
use crate::{Canvas, PixelationCamera};
use bevy::{prelude::*, render::primitives::Aabb};

/// Syncs the pixelation camera to the main camera.
pub(crate) fn sync_cameras<T: Component>(
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
pub(crate) fn position_canvas<T: Component>(
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
