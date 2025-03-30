use crate::ready_checks::{PixelationTargetKind, PixelationTargetReadyEvent};
use crate::PIXELATION_RENDER_LAYERS;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

pub(crate) fn recursively_set_layer(
    mut commands: Commands,
    mut ready_events: EventReader<PixelationTargetReadyEvent>,
    children: Query<&Children>,
    mesh_handles: Query<&Mesh3d>,
    scene_instances: Query<&SceneInstance>,
    scene_spawner: Res<SceneSpawner>,
) {
    for event in ready_events.read() {
        for (&entity, pixelation_target) in event.iter() {
            match pixelation_target.kind {
                PixelationTargetKind::Mesh => {
                    for child in children.iter_descendants(entity) {
                        if mesh_handles.contains(child) {
                            commands
                                .entity(child)
                                .insert(PIXELATION_RENDER_LAYERS.clone());
                        }
                    }
                }
                PixelationTargetKind::Scene => {
                    let scene_instance = scene_instances.get(entity).unwrap();
                    for child in scene_spawner.iter_instance_entities(**scene_instance) {
                        if mesh_handles.contains(child) {
                            commands
                                .entity(child)
                                .insert(PIXELATION_RENDER_LAYERS.clone());
                        }
                    }
                }
            }
        }
    }
}
