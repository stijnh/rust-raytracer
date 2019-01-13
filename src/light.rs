use crate::math::Vec3D;
use crate::texture::Color;
use rand::prelude::*;

pub trait Light: Send + Sync {
    fn sample_incidence(&self, pos: Vec3D, norm: Vec3D, rng: &mut SmallRng) -> (Vec3D, f32, Color);
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
    fn sample_incidence(&self, _: Vec3D, normal: Vec3D, _: &mut SmallRng) -> (Vec3D, f32, Color) {
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
        normal: Vec3D,
        _rng: &mut SmallRng,
    ) -> (Vec3D, f32, Color) {
        let offset = self.pos - pos;
        let dist_sq = offset.norm_squared();
        let dist = dist_sq.sqrt();
        let dir = offset / dist;
        let cos = Vec3D::dot(dir, normal).max(0.0);

        if self.radius == 0.0 {
            return (dir, dist, self.emission / dist_sq * cos);
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
    fn sample_incidence(
        &self,
        _: Vec3D,
        normal: Vec3D,
        _rng: &mut SmallRng,
    ) -> (Vec3D, f32, Color) {
        let cos = Vec3D::dot(-self.dir, normal).max(0.0);

        if self.cos_spread == 1.0 {
            return (-self.dir, 1e12, self.emission * cos);
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
    fn sample_incidence(&self, _: Vec3D, normal: Vec3D, rng: &mut SmallRng) -> (Vec3D, f32, Color) {
        let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let u = rng.gen::<f32>();
        let r = u.sqrt();

        let x = r * theta.cos();
        let y = r * theta.sin();
        let z = (1.0 - u).sqrt();

        let (a, b) = normal.ortho_axes();
        let dir = a * x + b * y + normal * z;

        (dir, self.dist, self.emission)
    }
}
