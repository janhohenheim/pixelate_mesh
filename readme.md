# Pixelate Mesh 
[![crates.io](https://img.shields.io/crates/v/pixelate_mesh)](https://crates.io/crates/pixelate_mesh)
[![docs.rs](https://docs.rs/pixelate_mesh/badge.svg)](https://docs.rs/pixelate_mesh)

Apply a pixelation effect to any Bevy mesh or scene without post-processing.

## Usage

- Add the `PixelateMeshPlugin`, where you specify a component that tracks the main camera.
- Add this tracking component to your camera.
- Add the `Pixelate` component to any entity that you want to pixelate.

The tracking component is needed because the plugin draws the textures on 2D canvases that need to rotate to always face the main camera.

## Compatibility
| bevy | pixelate_mesh |
|------|---------------|
| 0.10 | 0.1           |

## Examples
The following is an annotated minimal example. 
More can be found in the [examples folder](https://github.com/janhohenheim/pixelate_mesh/tree/main/examples).

```rust
use bevy::prelude::*;
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugin(PixelateMeshPlugin::<MainCamera>::default())
        .add_startup_system(setup)
        .run();
}

// Create a component for the main camera
#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        // This cube will render at 64x64 pixels
        Pixelate::splat(64),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
    ));

    commands.spawn((
        // Add the tracking component to the camera
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
```

## Inspiration

The plugin tries to emulate the effect as seen in Prodeus:  
<video src="https://www.youtube.com/watch?v=Vb-hPYOIwMw"></video>

## Shortcomings

The current setup does not work with multiple main cameras. Feel free to comment [on the issue](https://github.com/janhohenheim/pixelate_mesh/issues/1) if you have an idea for how to fix this!