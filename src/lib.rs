#![forbid(missing_docs)]
//! This crate provides a plugin for pixelating meshes.

use bevy::prelude::*;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{Pixelate, PixelateMeshPlugin};
}

mod creation;
mod ready_checks;
mod recursive_layering;
mod runtime;
mod util;

/// The plugin type for this crate.
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
            .add_event::<ready_checks::PixelationTargetReadyEvent>()
            .add_systems((
                ready_checks::get_ready_pixelation_targets,
                ready_checks::mark_for_pixelation,
                creation::add_pixelation,
                recursive_layering::recursively_set_layer,
                runtime::position_canvas::<C>,
                runtime::sync_cameras::<C>,
                runtime::despawn_dependent_types,
            ));
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

// Marks the main pass cube, to which the texture is applied.
#[derive(Debug, Component, Copy, Clone)]
struct Canvas {
    pub(crate) target: Entity,
}

#[derive(Debug, Component, Copy, Clone)]
struct PixelationCamera {
    pub(crate) target: Entity,
}
