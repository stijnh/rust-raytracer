#![allow(dead_code)]

#[macro_use]
mod common;
mod geom;
mod integrator;
mod light;
mod loader;
mod material;
mod math;
mod render;
mod scene;
mod texture;

use crate::geom::*;
use crate::math::*;
use crate::texture::*;
use crate::material::*;
use crate::light::*;
use std::sync::Arc;

pub fn main() {
    let mut list: Vec<Object> = vec![];
    let mesh = loader::load_ply_as_mesh("bunny_big.ply").unwrap();
    let mesh = Arc::new(Transform::new(mesh)
        .scale(20.0)
        .rotate_x(90f32.to_radians()));

    let bb = mesh.bounding_box();
    let cube = Cuboid::new(
        vec3d(-5.0, -5.0, bb.min[2]),
        vec3d(5.0, 5.0, bb.min[2] - 1.0),
    );

    list.push(Object::with_material(
            mesh.clone(), 
            Glass));

    list.push(Object::with_material(
            Translate::new(mesh.clone(), vec3d(0.0, -2.5, 0.0)),
            Metal));

    list.push(Object::with_material(
            Translate::new(mesh.clone(), vec3d(0.0, 2.5, 0.0)),
            Lambartian(COLOR_WHITE)));

    list.push(Object::with_material(cube, Glossy(1e3)));

    let obj_arc = Arc::new(geom::GeometryList::from_vec(list));
    let mut integrator = integrator::WhittedIntegrator::new();
    integrator.antialiasing = 4;
    integrator.shadow_rays = 20;

    let pos = Vec3D::new(-6.0, -6.0, 3.0);
    let up = -Vec3D::z_axis();
    let focus = Vec3D::new(-0.0, -0.0, 0.0);
    let cam = scene::Camera::new(1080, 960)
        .position(pos)
        .look_at(focus, up);

    let skybox = texture::Image::open("skybox.jpg").unwrap();

    let lights: Vec<Box<dyn light::Light>> = vec![
        Box::new(AmbientLight::new(Vec3D::one(), 0.1)),
        Box::new(DirectionLight::new(
            vec3d(-2.5, -3.0, -3.0),
            0.0,
            COLOR_WHITE,
            0.4,
        )),
        Box::new(AmbientOcclusion::new(1e12, Vec3D::one(), 0.5)),
    ];

    let scene = scene::Scene {
        root: obj_arc.clone(),
        camera: cam,
        skybox: Arc::new(skybox),
        lights,
    };

    for i in 5..100 {
        let mut fast_integrator = integrator.clone();
        fast_integrator.shadow_rays = i;
        fast_integrator.antialiasing = i;
        println!("{:?}", fast_integrator);
        let img = render::parallel_render_image(&scene, &fast_integrator);
        img.save(&format!("test_{}.png", i)).unwrap();
    }

    let img = render::parallel_render_image(&scene, &integrator);
    img.save("test.png").unwrap();
}
