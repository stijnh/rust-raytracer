use crate::texture::{Texture, Color, COLOR_GREEN, COLOR_BLACK, COLOR_WHITE};
use crate::math::*;
use rand::prelude::*;

pub static DEFAULT_MATERIAL: NullMaterial = NullMaterial;

pub trait Material: Send + Sync {
    fn sample_at(&self, _u: f32, _v: f32) -> Color {
        COLOR_BLACK
    }

    fn scatter(&self, _norm: Vec3D, _in: Vec3D, _rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        None
    }
}

pub struct NullMaterial;

impl Material for NullMaterial {
    fn sample_at(&self, _u: f32, _v: f32) -> Color {
        COLOR_GREEN
    }
}

pub struct Metal;

impl Material for Metal {
    fn scatter(&self, n: Vec3D, i: Vec3D, _: &mut SmallRng) -> Option<(Vec3D, Color)> {
        let out = i - 2.0 * n * Vec3D::dot(n, i);
        Some((out, COLOR_WHITE))
    }
}

pub struct Glossy(pub f32);

impl Material for Glossy {
    fn scatter(&self, n: Vec3D, i: Vec3D, rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        let out = i - 2.0 * n * Vec3D::dot(n, i);
        let (a, b) = out.ortho_axes();
        let side = iff!(Vec3D::dot(n, i) < 0.0, 1.0, -1.0);

        loop {
            let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let u = rng.gen::<f32>();

            let p = u.powf(2.0 / (1.0 + self.0));
            let r = (1.0 - p).sqrt();

            let x = r * theta.cos();
            let y = r * theta.sin();
            let z = p.sqrt();

            let o = x * a + y * b + z * out;

            if Vec3D::dot(n, out) * side > 0.0 {
                break Some((o, COLOR_WHITE));
            }
        }
    }
}

pub struct Glass;

impl Material for Glass {
    fn scatter(&self, n: Vec3D, i: Vec3D, rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        Transparent(1.5).scatter(n, i, rng)
    }
}


pub struct Transparent(f32);

impl Material for Transparent {
    fn scatter(&self, normal: Vec3D, i: Vec3D, rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        let side = Vec3D::dot(normal, i);
        let (c, eta, n) = if side < 0.0 {
            (-side, 1.0 / self.0, normal)
        } else {
            (side, self.0, -normal)
        };

        let k = 1.0 - eta * eta * (1.0 - c * c);

        let o = if k > 0.0 {
            eta * i + (eta * c - k.sqrt()) * n
        } else {
            i - 2.0 * n * Vec3D::dot(n, i)
        };

        Some((o, COLOR_WHITE))
    }
}

pub struct Lambartian<T: Texture>(pub T);

impl <T: Texture> Material for Lambartian<T> {
    fn sample_at(&self, u: f32, v: f32) -> Color {
        self.0.color_at(u, v)
    }
}
