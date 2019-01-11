use crate::math::Vec3D;
use crate::texture::Color;
use rand::prelude::*;

pub trait Light: Send + Sync {
    fn sample_incidence(&self, pos: Vec3D, norm: Vec3D, rng: &mut StdRng) -> (Vec3D, f32, Color);
    fn is_delta_distribution(&self) -> bool {
        false
    }
}

pub struct AmbientLight {
    emission: Color,
}

impl AmbientLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        AmbientLight {
            emission: color * intensity,
        }
    }
}

impl Light for AmbientLight {
    fn sample_incidence(&self, _: Vec3D, normal: Vec3D, _: &mut StdRng) -> (Vec3D, f32, Color) {
        (normal, 0.0, self.emission)
    }

    fn is_delta_distribution(&self) -> bool {
        true
    }
}

pub struct PointLight {
    pos: Vec3D,
    radius: f32,
    emission: Color,
}

impl PointLight {
    pub fn new(pos: Vec3D, radius: f32, color: Color, intensity: f32) -> Self {
        assert_eq!(radius, 0.0);

        PointLight {
            pos,
            radius,
            emission: color * intensity,
        }
    }
}

impl Light for PointLight {
    fn sample_incidence(
        &self,
        pos: Vec3D,
        _: Vec3D,
        _rng: &mut StdRng,
    ) -> (Vec3D, f32, Color) {
        let offset = self.pos - pos;
        let dist_sq = offset.norm_squared();
        let dist = dist_sq.sqrt();
        let dir = offset / dist;

        if self.radius == 0.0 {
            return (dir, dist, self.emission / dist_sq);
        }

        panic!("unreachable");
    }

    fn is_delta_distribution(&self) -> bool {
        self.radius == 0.0
    }
}

pub struct DirectionLight {
    dir: Vec3D,
    cos_spread: f32,
    emission: Color,
}

impl DirectionLight {
    pub fn new(dir: Vec3D, spread: f32, color: Color, intensity: f32) -> Self {
        assert_eq!(spread, 0.0);

        DirectionLight {
            dir: dir.normalize(),
            cos_spread: spread.cos(),
            emission: color * intensity,
        }
    }
}

impl Light for DirectionLight {
    fn sample_incidence(&self, _: Vec3D, _: Vec3D, _rng: &mut StdRng) -> (Vec3D, f32, Color) {
        if self.cos_spread == 1.0 {
            return (-self.dir, 1e12, self.emission);
        }

        panic!("unreachable");
    }

    fn is_delta_distribution(&self) -> bool {
        self.cos_spread == 1.0
    }
}

pub struct AmbientOcclusion {
    dist: f32,
    emission: Color,
}

impl AmbientOcclusion {
    pub fn new(dist: f32, color: Color, intensity: f32) -> Self {
        Self {
            dist: dist.abs(),
            emission: color * intensity,
        }
    }
}

impl Light for AmbientOcclusion {
    fn sample_incidence(&self, _: Vec3D, normal: Vec3D, rng: &mut StdRng) -> (Vec3D, f32, Color) {
        loop {
            let dir = Vec3D::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let length_sq = dir.norm_squared();
            let cos = Vec3D::dot(normal, dir);

            if cos > 0.0 && length_sq > 1e-12 && length_sq < 1.0 {
                let length = length_sq.sqrt();
                break (dir / length.sqrt(), self.dist, self.emission);
            }
        }
    }
}
