use bevy::{render::primitives::Aabb};

pub(crate) fn get_max_radius(aabb: &Aabb) -> f32 {
    aabb.half_extents.length()
}
