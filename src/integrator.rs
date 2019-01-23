use crate::light::Light;
use crate::math::*;
use crate::scene::Scene;
use crate::texture::Color;
use crate::material::Material;
use rand::prelude::*;
use std::f32;

use std::sync::{atomic::{Ordering::SeqCst, AtomicUsize}, Arc};

#[derive(Clone, Debug)]
pub struct WhittedIntegrator {
    pub max_depth: i32,
    pub shadow_rays: i32,
    pub scatter_rays: i32,
    pub antialiasing: i32,
    pub gamma: f32,
    pub dd: Arc<AtomicUsize>,
}

impl WhittedIntegrator {
    pub fn new() -> Self {
        Self {
            max_depth: 10,
            shadow_rays: 5,
            scatter_rays: 1,
            antialiasing: 1,
            gamma: 2.2,
            dd: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn calculate_pixel(&self, scene: &Scene, cx: usize, cy: usize) -> Color {
        let n = self.antialiasing;
        let mut pixels = vec![];
        let mut rng = SmallRng::seed_from_u64((cx.to_le() ^ cy.to_be()) as u64);
        self.dd.store(0, SeqCst);
        
        for i in 0..n {
            for j in 0..n {
                let x = (cx as f32) + (i as f32 + 0.5) / n as f32 - 0.5;
                let y = (cy as f32) + (j as f32 + 0.5) / n as f32 - 0.5;

                let ray = scene.camera.generate_ray(x, y);
                let pixel = self.integrate_recur(scene, &ray, 0, &mut rng);
                pixels.push(pixel);
            }
        }

        let len = pixels.len() as f32;
        pixels
            .into_iter()
            .fold(Vec3D::zero(), |a, b| a + b) / len

        //Color::fill(self.dd.load(SeqCst) as f32 / 1000.0)
    }

    fn integrate_recur(&self, scene: &Scene, ray: &Ray, depth: i32, rng: &mut SmallRng) -> Color {
        if depth >= self.max_depth {
            return scene.calculate_background(ray);
        }
        //self.dd.fetch_add(1, SeqCst);

        let hit = match scene.root.hit(ray, 1e12) {
            Some(x) => x,
            None => return scene.calculate_background(ray),
        };


        let mut color = Color::zero();
        let [u, v] = hit.uv;
        let n = hit.norm.normalize();
        let outside = Vec3D::dot(n, -ray.dir) > 0.0;
        let p_out = hit.pos + n * 0.001;
        let p_in = hit.pos - n * 0.001;
        let p = iff!(outside, p_out, p_in);

        let diffuse = hit.material.sample_at(u, v);

        if !diffuse.is_zero() {
            let mut illumination = Vec3D::zero();

            for light in &scene.lights {
                illumination += self.illumination(scene, &**light, p, n, rng);
            }

            color += diffuse * illumination;
        }

        for _ in 0..self.scatter_rays {
            let scatter = hit.material.scatter(n, ray.dir, rng);

            if let Some((out, scatter)) = scatter {
                let p = iff!(Vec3D::dot(out, n) > 0.0, p_out, p_in);

                color += scatter * self.integrate_recur(
                    scene,
                    &Ray::new(p, out),
                    depth + 1,
                    rng) / (self.scatter_rays as f32);
            }
        }

        color
    }

    fn illumination(
        &self,
        scene: &Scene,
        light: &dyn Light,
        pos: Vec3D,
        normal: Vec3D,
        rng: &mut SmallRng,
    ) -> Vec3D {
        let mut total = Color::zero();
        let n = iff!(light.is_delta_distribution(), 1, self.shadow_rays);

        for _ in 0..n {
            let (dir, t_max, ill) = light.sample_incidence(pos, normal, rng);
            let ray = Ray::new(pos, dir);

            //println!("{:?} {:?} {:?} {:?}", dir, ill, t_max, Vec3D::dot(dir, normal));

            if t_max == 0.0 || !scene.root.is_hit(&ray, t_max) {
                total += ill;
            }
        }

        //println!("total={} {:?}", n, total / (n as f32));
        //println!("");
        total / n as f32
    }
}
