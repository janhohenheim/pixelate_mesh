use crate::ready_checks::{PixelationTargetKind, PixelationTargetReadyEvent};
use bevy::pbr::NotShadowReceiver;
use bevy::platform_support::collections::HashSet;
use bevy::prelude::*;
use bevy::scene::InstanceId;

#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub(crate) struct ShadowMaterialHandle(Handle<StandardMaterial>);

#[derive(Debug, Clone, Resource, Deref, DerefMut, Default)]
pub(crate) struct SetSceneShadow(HashSet<InstanceId>);

pub(crate) fn create_shadow_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handle = materials.add(StandardMaterial {
        base_color: Color::NONE,
        unlit: true,
        alpha_mode: AlphaMode::Multiply,
        ..Default::default()
    });
    commands.insert_resource(ShadowMaterialHandle(handle));
}

pub(crate) fn add_shadow_caster(
    mut commands: Commands,
    mut ready_event: EventReader<PixelationTargetReadyEvent>,
    mesh_handles: Query<&Mesh3d>,
    children: Query<&Children>,
    scene_handles: Query<&SceneRoot>,
    mut scene_spawner: ResMut<SceneSpawner>,
    shadow_material_handle: Res<ShadowMaterialHandle>,
    mut set_scene_shadow: ResMut<SetSceneShadow>,
) {
    for event in ready_event.read() {
        for (&entity, target) in event.iter() {
            match target.kind {
                PixelationTargetKind::Mesh => {
                    commands.entity(entity).with_children(|parent| {
                        parent
                            .spawn((
                                Name::new("Pixelation Shadow"),
                                Transform::default(),
                                Visibility::default(),
                            ))
                            .with_children(|parent| {
                                duplicate_children(
                                    entity,
                                    parent,
                                    &children,
                                    &mesh_handles,
                                    &shadow_material_handle,
                                );
                            });
                    });
                }
                PixelationTargetKind::Scene => {
                    let scene_handle = scene_handles.get(entity).unwrap();
                    let instance_id = scene_spawner.spawn_as_child(scene_handle.0.clone(), entity);
                    set_scene_shadow.insert(instance_id);
                }
            }
        }
    }
}

fn duplicate_children(
    entity: Entity,
    child_builder: &mut ChildSpawnerCommands,
    children: &Query<&Children>,
    mesh_handles: &Query<&Mesh3d>,
    shadow_material_handle: &Handle<StandardMaterial>,
) {
    let mut entity_commands = child_builder.spawn_empty();
    if let Ok(mesh_handle) = mesh_handles.get(entity) {
        entity_commands.insert((
            mesh_handle.clone(),
            MeshMaterial3d(shadow_material_handle.clone()),
        ));
    }
    let children_entities = match children.get(entity) {
        Ok(children) => children,
        _ => {
            return;
        }
    };
    for child in children_entities.iter() {
        entity_commands.with_children(|parent| {
            duplicate_children(
                child,
                parent,
                children,
                mesh_handles,
                shadow_material_handle,
            );
        });
    }
}

pub(crate) fn set_scene_shadow(
    mut commands: Commands,
    mut set_scene_shadow: ResMut<SetSceneShadow>,
    scene_spawner: Res<SceneSpawner>,
    shadow_material_handle: Res<ShadowMaterialHandle>,
    mesh_query: Query<&Mesh3d>,
) {
    let mut done = HashSet::default();
    for instance_id in set_scene_shadow.iter() {
        if scene_spawner.instance_is_ready(*instance_id) {
            done.insert(*instance_id);
        }
    }
    set_scene_shadow.0 = set_scene_shadow.0.difference(&done).copied().collect();
    for instance_id in done {
        for entity in scene_spawner.iter_instance_entities(instance_id) {
            if mesh_query.contains(entity) {
                commands.entity(entity).insert((
                    NotShadowReceiver,
                    MeshMaterial3d(shadow_material_handle.0.clone()),
                ));
            }
        }
    }
}
