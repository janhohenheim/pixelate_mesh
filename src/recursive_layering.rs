use crate::ready_checks::{PixelationTargetKind, PixelationTargetReadyEvent};
use crate::util::get_pixelation_render_layer;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

pub(crate) fn recursively_set_layer(
    mut commands: Commands,
    mut ready_events: EventReader<PixelationTargetReadyEvent>,
    children: Query<&Children>,
    mesh_handles: Query<&Handle<Mesh>>,
    scene_instances: Query<&SceneInstance>,
    scene_spawner: Res<SceneSpawner>,
) {
    let first_pass_layer = get_pixelation_render_layer();
    for event in ready_events.iter() {
        for (&entity, pixelation_target) in event.iter() {
            match pixelation_target.kind {
                PixelationTargetKind::Mesh => {
                    for child in children.iter_descendants(entity) {
                        if mesh_handles.contains(child) {
                            commands.entity(child).insert(first_pass_layer);
                        }
                    }
                }
                PixelationTargetKind::Scene => {
                    let scene_instance = scene_instances.get(entity).unwrap();
                    for child in scene_spawner.iter_instance_entities(**scene_instance) {
                        if mesh_handles.contains(child) {
                            commands.entity(child).insert(first_pass_layer);
                        }
                    }
                }
            }
        }
    }
}
