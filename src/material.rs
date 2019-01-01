use crate::texture::{Texture, COLOR_GREEN};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref DEFAULT_MATERIAL: Material = {
        Material {
            texture: Arc::new(COLOR_GREEN),
            diffuse: 0.0,
        }
    };
}

pub struct Material {
    pub texture: Arc<dyn Texture>,
    pub diffuse: f32,
}
