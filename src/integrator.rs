use crate::light::Light;
use crate::math::*;
use crate::scene::Scene;
use crate::texture::Color;
use rand::prelude::*;
use std::f32;

pub struct WhittedIntegrator {
    pub max_depth: i32,
    pub shadow_rays: i32,
    pub antialiasing: i32,
    pub gamma: f32,
}

impl WhittedIntegrator {
    pub fn new() -> Self {
        Self {
            max_depth: 10,
            shadow_rays: 5,
            antialiasing: 1,
            gamma: 2.2,
        }
    }

    pub fn calculate_pixel(&self, scene: &Scene, cx: usize, cy: usize) -> Color {
        let n = self.antialiasing;
        let mut pixel = Vec3D::zero();
        let mut rng = StdRng::seed_from_u64(0);

        for i in 0..n {
            for j in 0..n {
                let x = (cx as f32) + (i as f32 + 0.5) / n as f32 - 0.5;
                let y = (cy as f32) + (j as f32 + 0.5) / n as f32 - 0.5;

                let ray = scene.camera.generate_ray(x, y);
                pixel += self.integrate_recur(scene, &ray, 0, &mut rng);
            }
        }

        (pixel / (n as f32 * n as f32)).map(|x| x.powf(1.0 / self.gamma))
    }

    fn integrate_recur(&self, scene: &Scene, ray: &Ray, depth: i32, rng: &mut StdRng) -> Color {
        if depth >= self.max_depth {
            return scene.calculate_background(ray);
        }

        let hit = match scene.root.hit(ray, 1e12) {
            Some(x) => x,
            None => return scene.calculate_background(ray),
        };

        let [u, v] = hit.uv;
        let attenuation = hit.material.texture.color_at(u, v);

        let n = hit.norm.normalize();
        let p = hit.pos + n * 0.01;
        let mut illumination = Vec3D::zero();

        for light in &scene.lights {
            illumination += self.illumination(scene, &**light, p, n, rng);
        }

        illumination * attenuation
    }

    fn illumination(
        &self,
        scene: &Scene,
        light: &dyn Light,
        pos: Vec3D,
        normal: Vec3D,
        rng: &mut StdRng,
    ) -> Vec3D {
        let mut total = Color::zero();
        let n = iff!(light.is_delta_distribution(), 1, self.shadow_rays);

        for _ in 0..n {
            let (dir, t_max, ill) = light.sample_incidence(pos, normal, rng);
            let cos = Vec3D::dot(dir, normal);
            let ray = Ray::new(pos, dir);

            if cos > 0.0 && (t_max == 0.0 || !scene.root.is_hit(&ray, t_max)) {
                total += cos * ill;
            }
        }

        total / n as f32
    }
}
