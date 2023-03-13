use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EditorPlugin)
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(setup)
        .add_system(change_pixelation)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Cube"),
        Pixelate::splat(64),
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

fn change_pixelation(time: Res<Time>, mut query: Query<&mut Pixelate>) {
    for mut pixelate in query.iter_mut() {
        let factor = (time.elapsed_seconds() / 2.).sin().abs();
        let factor = factor * factor;
        let new_pixelation = (factor * 150.).clamp(4., 128.);
        *pixelate = Pixelate::splat(new_pixelation as u32);
    }
}
