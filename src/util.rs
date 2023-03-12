use bevy::render::primitives::Aabb;
use bevy::render::view::RenderLayers;

pub(crate) fn get_max_radius(aabb: &Aabb) -> f32 {
    aabb.half_extents.length()
}

pub(crate) fn get_pixelation_render_layer() -> RenderLayers {
    RenderLayers::layer(1)
}
