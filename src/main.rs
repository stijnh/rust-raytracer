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

use crate::math::*;
use crate::geom::*;
use crate::texture::*;
use std::sync::Arc;

pub fn main() {
    let mut list: Vec<Box<dyn Geometry>> = vec![];
    let mesh = loader::load_ply_as_mesh("bunny_big.ply").unwrap();
    let mesh = Transform::new(mesh)
        .scale(20.0)
        .rotate_x(90f32.to_radians());

    let bb = mesh.bounding_box();
    println!("{:?}", bb);
    let cube = Cuboid::new(
        vec3d(-5.0, -5.0, bb.min[2]),
        vec3d(5.0, 5.0, bb.min[2] - 1.0),
    );


    list.push(Box::new(mesh));
    list.push(Box::new(cube));

    let obj_arc = Arc::new(geom::GeometryList::from_vec(list));
    let mut integrator = integrator::WhittedIntegrator::new();
    integrator.antialiasing = 4;
    integrator.shadow_rays = 20;

    let pos = Vec3D::new(-2.5, -3.0, 3.0);
    let up = -Vec3D::z_axis();
    let focus = Vec3D::new(-0.0, -0.0, 0.0);
    let cam = scene::Camera::new(800, 600)
        .position(pos)
        .look_at(focus, up);

    let skybox = texture::Image::open("skybox.jpg").unwrap();

    let lights: Vec<Box<dyn light::Light>> = vec![
        //Box::new(light::AmbientLight::new(Vec3D::one(), 0.0)),
        //Box::new(light::DirectionLight::new(vec3d(-1.0, -1.0, -1.0), 0.0, Vec3D::one(), 1.0)),
        /*
        Box::new(light::PointLight::new(
            pos, //Vec3D::new(-0.0, 0.0, 3.0),
            0.0,
            vec3d(1.0, 1.0, 1.0),
            10.0,
        )),
        */
        Box::new(
            light::DirectionLight::new(
                vec3d(-2.5, -3.0, -3.0),
                0.0,
                COLOR_WHITE,
                0.5)
        ),

        Box::new(
            light::AmbientOcclusion::new(0.3, Vec3D::one(), 0.5)
        ),
        //scene::Light::new_directional(Vec3D::fill(0.45), vec3d(1.0, 1.0, 1.0)),
        //scene::Light::new_directional(Vec3D::fill(0.45), vec3d(-1.0, 1.0, 1.0)),
    ];

    let scene = scene::Scene {
        root: obj_arc.clone(),
        camera: cam,
        skybox: Arc::new(skybox),
        lights,
    };


    for i in 1..10 {
        let mut fast_integrator = integrator.clone();
        fast_integrator.shadow_rays = i * 10;
        fast_integrator.antialiasing = 1;
        println!("{:?}", fast_integrator);
        let img = render::parallel_render_image(&scene, &fast_integrator);
        img.save(&format!("test_{}.png", i)).unwrap();
    }

    let img = render::parallel_render_image(&scene, &integrator);
    img.save("test.png").unwrap();
}
