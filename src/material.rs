use texture::Texture;
use util::Color;
use std::sync::Arc;
use std::mem::zeroed;

pub static DEFAULT_MATERIAL: NullMaterial = NullMaterial;

pub trait Material: Send + Sync {
    fn get(&self, uv: (f32, f32)) -> (Color, Color);
}

pub struct ReflectionMaterial;

impl Material for ReflectionMaterial {
    fn get(&self, uv: (f32, f32)) -> (Color, Color) {
        (Color::zero(), Color::one())
    }
}

pub struct LambertianMaterial {
    texture: Arc<Texture>,
}

impl LambertianMaterial {
    pub fn new<T: 'static>(texture: T) -> Self 
            where T: Texture 
    {
        LambertianMaterial { 
            texture: Arc::new(texture)
        }
    }
}

impl Material for LambertianMaterial {
    fn get(&self, (u, v): (f32, f32)) -> (Color, Color) {
        (self.texture.color_at(u, v), Color::zero())
    }
}

pub struct NullMaterial;

impl Material for NullMaterial {
    fn get(&self, _: (f32, f32)) -> (Color, Color) {
        (Color::one(), Color::zero())
    }
}
