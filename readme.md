> [!IMPORTANT]
> This plugin is no longer maintained, as I have come to believe that the approach used here is not correct.
> There's quite a few issues with visibility around corners. I think deferred rendering would solve this,
> but I don't have the time to properly implement that properly.
> 
> If you still want a simple way to achieve the effects shown in the readme and examples,
> I recommend following the official [Pixed Grid Snapping](https://bevyengine.org/examples/2d-rendering/pixel-grid-snap/) example, as that is quite easy to implement.
> If you really need this plugin to still exist, ping me on Discord or send me an email and I'll update it again :)

# Pixelate Mesh

[![crates.io](https://img.shields.io/crates/v/pixelate_mesh)](https://crates.io/crates/pixelate_mesh)
[![docs.rs](https://docs.rs/pixelate_mesh/badge.svg)](https://docs.rs/pixelate_mesh)

Apply a pixelation effect to any Bevy mesh or scene without post-processing.

![Pixelated foxes](./docs/foxes.jpg?raw=true "Pixelated Foxes")

## Usage

- Add the `PixelateMeshPlugin`, where you specify a component that tracks the main camera.
- Add this tracking component to your camera.
- Add the `Pixelate` component to any entity that you want to pixelate.

The tracking component is needed because the plugin draws the textures on 2D canvases that need to rotate to always face
the main camera.

## Compatibility

| bevy        | pixelate_mesh |
|-------------|---------------|
| 0.16.0-rc   | 0.6.rc        |
| 0.15        | 0.5           |
| 0.14        | 0.4           |
| 0.13        | 0.3           |
| 0.12        | 0.2           |
| 0.10        | 0.1           |

## Examples

The following is an annotated minimal example.
More can be found in the [examples folder](./examples).

```rust,no_run
use bevy::prelude::*;
use pixelate_mesh::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(PixelateMeshPlugin::<MainCamera>::default())
        .add_systems(Startup, setup)
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
        Mesh3d(meshes.add(Mesh::from(Cuboid::from_length(1.0)))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    commands.spawn((
        // Add the tracking component to the camera
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Important, otherwise it won't look pixelated!
        Msaa::Off,
    ));
}
```

## Inspiration

The plugin tries to emulate the effect as seen in Prodeus:  
<video src="https://user-images.githubusercontent.com/9047632/224768897-f50f15fc-50ab-49a9-8c77-a33ef01fad5b.mp4"></video>
[Source](https://www.youtube.com/watch?v=Vb-hPYOIwMw)

## Shortcomings

- The current setup does not work with multiple main cameras. Feel free to
  comment [on the issue](https://github.com/janhohenheim/pixelate_mesh/issues/1) if you have an idea for how to fix
  this!
