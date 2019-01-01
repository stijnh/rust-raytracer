#![allow(dead_code)]

#[macro_use]
mod common;
mod geom;
mod integrator;
mod material;
mod math;
mod render;
mod scene;
mod texture;

use crate::math::*;
use std::sync::Arc;

pub fn main() {
    let mut obj = geom::GeometryList::new();
    for j in -10..=20 {
        for i in -10..=20 {
            obj.push(geom::Sphere::new(vec3d(j as f32, i as f32, 0.0), 0.3));
        }
    }

    let obj_arc = Arc::new(obj);
    let mut integrator = integrator::WhittedIntegrator::new();
    integrator.antialiasing = 1;

    let pos = Vec3D::new(-5.0, -6.0, 3.0);
    let up = -Vec3D::z_axis();
    let focus = Vec3D::new(-4.0, -4.0, 0.0);
    let cam = scene::Camera::new(800, 600)
        .position(pos)
        .look_at(focus, up);

    let skybox = texture::Image::open("skybox.jpg").unwrap();

    let lights = vec![
        scene::Light::new_ambient(Vec3D::fill(0.1)),
        scene::Light::new_directional(Vec3D::fill(0.45), vec3d(1.0, 1.0, 1.0)),
        scene::Light::new_directional(Vec3D::fill(0.45), vec3d(-1.0, 1.0, 1.0)),
    ];

    let scene = scene::Scene {
        root: obj_arc.clone(),
        camera: cam,
        skybox: Arc::new(skybox),
        lights,
    };

    let img = render::parallel_render_image(&scene, &integrator);
    img.save("test.png").unwrap();
}
