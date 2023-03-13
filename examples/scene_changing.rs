use bevy::prelude::*;
use pixelate_mesh::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(setup)
        .add_system(change_pixelation)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Fox"),
        Pixelate::splat(128),
        SceneBundle {
            scene: asset_server.load("Fox.glb#Scene0"),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(100.0, 100.0, 150.0)
                .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Light"),
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::ZYX,
                0.0,
                1.0,
                -PI / 4.,
            )),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));
}

fn change_pixelation(time: Res<Time>, mut query: Query<&mut Pixelate>) {
    for mut pixelate in query.iter_mut() {
        let factor = (time.elapsed_seconds() / 2.).sin().abs();
        let factor = factor * factor;
        let new_pixelation = (factor * 600.).clamp(32., 512.);
        *pixelate = Pixelate::splat(new_pixelation as u32);
    }
}
