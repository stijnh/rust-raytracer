use crate::texture::{Texture, Color, COLOR_GREEN, COLOR_BLACK};
use crate::math::*;
use std::sync::Arc;

pub static DEFAULT_MATERIAL: NullMaterial = NullMaterial;

pub trait Material: Send + Sync {
    fn sample_at(&self, u: f32, v: f32) -> (Color, Vec3D, Color);
}

pub struct NullMaterial;

impl Material for NullMaterial {
    fn sample_at(&self, u: f32, v: f32) -> (Color, Vec3D, Color) {
        (COLOR_GREEN, Vec3D::zero(), COLOR_BLACK)
    }
}
