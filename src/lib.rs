#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![forbid(missing_docs)]
//! Apply a pixelation effect to any Bevy mesh or scene without post-processing.
//!
//! ## Usage
//!
//! - Add the `PixelateMeshPlugin`, where you specify a component that tracks the main camera.
//! - Add this tracking component to your camera.
//! - Add the `Pixelate` component to any entity that you want to pixelate.
//!
//! The tracking component is needed because the plugin draws the textures on 2D canvases that need to rotate to always face the main camera.
//!
//! ## Example
//! The following is an annotated minimal example.
//! More can be found in the [examples folder](https://github.com/janhohenheim/pixelate_mesh/tree/main/examples).
//!
//! ```ignore
//! use bevy::prelude::*;
//! use pixelate_mesh::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Add the plugin
//!         .add_plugins(PixelateMeshPlugin::<MainCamera>::default())
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! // Create a component for the main camera
//! #[derive(Component)]
//! struct MainCamera;
//!
//! fn setup(
//!     mut commands: Commands,
//!     mut meshes: ResMut<Assets<Mesh>>,
//!     mut materials: ResMut<Assets<StandardMaterial>>,
//! ) {
//!     commands.spawn((
//!         // This cube will render at 64x64 pixels
//!         Pixelate::splat(64),
//!         PbrBundle {
//!             mesh: meshes.add(Mesh::from(Cuboid::default())),
//!             material: materials.add(StandardMaterial::from(Color::WHITE)),
//!             ..default()
//!         },
//!     ));
//!
//!     commands.spawn((
//!         // Add the tracking component to the camera
//!         MainCamera,
//!         Camera3dBundle {
//!             transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//!             ..default()
//!         },
//!     ));
//! }
//! ```
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{Pixelate, PixelateMeshPlugin, PIXELATION_RENDER_LAYERS};
}

mod creation;
mod ready_checks;
mod recursive_layering;
mod runtime;
mod shadow;
mod util;

/// The plugin type for this crate.
/// The generic parameter `C` is the type of the component that tracks the main camera.
#[derive(Debug)]
pub struct PixelateMeshPlugin<C: Component> {
    _camera_type: std::marker::PhantomData<C>,
}

impl<C: Component> Default for PixelateMeshPlugin<C> {
    fn default() -> Self {
        Self {
            _camera_type: std::marker::PhantomData,
        }
    }
}

impl<C> Plugin for PixelateMeshPlugin<C>
where
    C: Component,
{
    fn build(&self, app: &mut App) {
        app.register_type::<Pixelate>()
            .init_resource::<ready_checks::ToPixelate>()
            .init_resource::<creation::Ordering>()
            .init_resource::<shadow::SetSceneShadow>()
            .add_event::<ready_checks::PixelationTargetReadyEvent>()
            .add_systems(Startup, shadow::create_shadow_material)
            .add_systems(
                Update,
                (
                    ready_checks::get_ready_pixelation_targets,
                    ready_checks::mark_for_pixelation,
                    creation::add_pixelation,
                    recursive_layering::recursively_set_layer,
                    shadow::add_shadow_caster,
                    shadow::set_scene_shadow,
                    runtime::update_pixelation,
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    runtime::position_canvas::<C>,
                    runtime::sync_cameras::<C>,
                    runtime::despawn_dependent_types,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, runtime::set_visible);
    }
}

/// Marks the entity containing a mesh to be pixelated.
#[derive(Debug, Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct Pixelate {
    /// How many pixels wide the final pixelated image should be.
    pub horizontal_pixels: u32,
    /// How many pixels tall the final pixelated image should be.
    pub vertical_pixels: u32,
}

impl Pixelate {
    /// Creates a new `Pixelate` component with the given dimensions.
    pub fn splat(horizontal_and_vertical_pixels: u32) -> Self {
        Self {
            horizontal_pixels: horizontal_and_vertical_pixels,
            vertical_pixels: horizontal_and_vertical_pixels,
        }
    }
}

/// Marks the main pass plane, to which the texture is applied.
#[derive(Debug, Component, Copy, Clone)]
struct Canvas {
    pub(crate) target: Entity,
}

#[derive(Debug, Component, Copy, Clone)]
struct PixelationCamera {
    pub(crate) target: Entity,
}

/// The render layers used by the plugin. All objects that will be pixelated are rendered on these layers.
/// If you want light to affect them, you need to add the light to the same layers.
pub const PIXELATION_RENDER_LAYERS: RenderLayers = RenderLayers::layer(1);

#[cfg(doctest)]
#[doc = include_str!("../readme.md")]
mod test_readme {}
