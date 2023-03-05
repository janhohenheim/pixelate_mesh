use bevy::prelude::*;
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .insert_resource(Msaa { samples: 1 })
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
}
