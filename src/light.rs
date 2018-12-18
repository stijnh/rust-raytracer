use math::{Vec3D, Ray};
use rand::Rng;

pub struct Light {
    pos: Vec3D,
    radius: f32
}

impl Light {
    pub fn new(pos: Vec3D, radius: f32) -> Self {
        Light {
            pos,
            radius
        }
    }

    pub fn generate_ray<R: Rng>(&self, dest: Vec3D, rng: &mut R) -> (Ray, f32, f32) {
        loop {
            let p = Vec3D::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0));
            let norm = p.length();
        
            if norm > 0.0 && norm < 1.0 {
                let q = self.pos + p * self.radius;
                let dir = q - dest;
                let dir_norm = dir.length();

                break (Ray::new(dest, dir / dir_norm), 0.01, dir_norm);
            }
        }
    }
}
