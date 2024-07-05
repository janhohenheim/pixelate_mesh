use bevy::prelude::*;
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(setup)
        .add_system(toggle)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Pixelatable;

fn toggle(
    mut commands: Commands,
    query: Query<Entity, With<Pixelatable>>,
    mut pixelate_state: Local<bool>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *pixelate_state = !*pixelate_state;
        for pixelated_entity in &query {
            if *pixelate_state {
                commands
                    .entity(pixelated_entity)
                    .insert(Pixelate::splat(64));
            } else {
                commands.entity(pixelated_entity).remove::<Pixelate>();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Cube"),
        Pixelatable,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
            ..default()
        },
    ));
}
