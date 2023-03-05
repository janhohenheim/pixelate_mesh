#![forbid(missing_docs)]
//! This crate provides a plugin for pixelating meshes.

use bevy::prelude::*;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{Pixelate, PixelateMeshPlugin};
}

mod creation;
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
            .add_system(runtime::position_canvas::<C>)
            .add_system(runtime::sync_cameras::<C>)
            .add_system(runtime::despawn_dependent_types)
            .add_system(creation::add_pixelation);
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

// Marks the main pass cube, to which the texture is applied.
#[derive(Debug, Component, Copy, Clone)]
struct Canvas {
    pub(crate) target: Entity,
}

#[derive(Debug, Component, Copy, Clone)]
struct PixelationCamera {
    pub(crate) target: Entity,
}
