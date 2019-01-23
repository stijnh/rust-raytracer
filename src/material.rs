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
    fn sample_at(&self, _u: f32, _v: f32) -> Color {
        COLOR_BLACK
    }

    fn scatter(&self, n: Vec3D, i: Vec3D, _: &mut SmallRng) -> Option<(Vec3D, Color)> {
        let out = i - 2.0 * n * Vec3D::dot(n, i);
        Some((out, COLOR_WHITE))
    }
}

pub struct Glossy<T: Texture>(pub f32, pub f32, pub T);

impl <T: Texture> Material for Glossy<T> {
    fn sample_at(&self, u: f32, v: f32) -> Color {
        self.2.color_at(u, v) * (1.0 - self.1)
    }

    fn scatter(&self, n: Vec3D, i: Vec3D, rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        let out = reflection(n, i);
        let (a, b) = out.ortho_axes();
        let side = iff!(Vec3D::dot(n, i) < 0.0, 1.0, -1.0);

        for _ in 0..10 {
            let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let u = rng.gen::<f32>();

            let p = u.powf(2.0 / (1.0 + self.0));
            let r = (1.0 - p).sqrt();

            let x = r * theta.cos();
            let y = r * theta.sin();
            let z = p.sqrt();

            let o = x * a + y * b + z * out;

            if Vec3D::dot(n, out) * side > 0.0 {
                return Some((o, COLOR_WHITE * self.1));
            }
        }

        None
    }
}

pub struct Glass;

impl Material for Glass {
    fn scatter(&self, n: Vec3D, i: Vec3D, rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        Transparent(1.5).scatter(n, i, rng)
    }
}


pub struct Transparent(pub f32);

pub fn reflection(normal: Vec3D, i: Vec3D) -> Vec3D {
    i - 2.0 * normal * Vec3D::dot(normal, i)
}

pub fn refraction(normal: Vec3D, i: Vec3D, ior: f32) -> Option<Vec3D> {
    let side = Vec3D::dot(normal, i);
    let (cosi, eta, n) = if side < 0.0 {
        (-side, 1.0 / ior, normal)
    } else {
        (side, ior, -normal)
    };

    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k > 0.0 {
        Some(eta * i + (eta * cosi - k.sqrt()) * n)
    } else {
        None
    }
}

pub fn fresnel(normal: Vec3D, i: Vec3D, ior: f32) -> f32 {
    let side = Vec3D::dot(normal, i);
    let (cosi, etat, etai) = if side < 0.0 {
        (-side, ior, 1.0)
    } else {
        (side, 1.0, ior)
    };

    let sint = etai / etat * (1.0 - cosi * cosi).sqrt();

    if sint < 1.0 {
        let cost = (1.0 - sint * sint).sqrt();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etat * cosi) - (etai * cost)) / ((etai * cosi) + (etat * cost));
        (rs * rs + rp * rp) / 2.0
    } else {
        1.0
    }
}

impl Material for Transparent {
    fn scatter(&self, normal: Vec3D, i: Vec3D, rng: &mut SmallRng) -> Option<(Vec3D, Color)> {
        let o = if rng.gen::<f32>() < fresnel(normal, i, self.0) {
            reflection(normal, i)
        } else {
            refraction(normal, i, self.0)?
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
