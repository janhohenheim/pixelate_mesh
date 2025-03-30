use bevy::prelude::*;
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelateMeshPlugin::<MainCamera>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, change_pixelation)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Fox"),
        Pixelate::splat(128),
        SceneRoot(asset_server.load("Fox.glb#Scene0")),
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(100.0, 100.0, 150.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        Name::new("Light"),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        PIXELATION_RENDER_LAYERS.clone(),
    ));
}

fn change_pixelation(time: Res<Time>, mut query: Query<&mut Pixelate>) {
    for mut pixelate in query.iter_mut() {
        let factor = (time.elapsed_secs() / 2.).sin().abs();
        let factor = factor * factor;
        let new_pixelation = (factor * 600.).clamp(32., 512.);
        *pixelate = Pixelate::splat(new_pixelation as u32);
    }
}
