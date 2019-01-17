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
    let mesh = loader::load_ply_as_mesh("../data/bunny_big.ply").unwrap();
    let mesh = Arc::new(Transform::new(mesh)
        .scale(20.0)
        .rotate_x(90f32.to_radians()));

    let bb = mesh.bounding_box();
    let cube = Cuboid::new(
        vec3d(-5.0, -5.0, bb.min[2]),
        vec3d(5.0, 5.0, bb.min[2] - 1.0),
    );

    let c = (bb.min + bb.max) / 2.0;
    let r = 1.0;
    //let mesh = Sphere::new(c, r);

    list.push(Object::with_material(
            mesh.clone(), 
            Transparent(1.0 / 0.6)));

    list.push(Object::with_material(
            Translate::new(mesh.clone()).translate_y(-2.5),
            Metal));

    list.push(Object::with_material(
            Translate::new(mesh.clone()).translate_y(2.5),
            Lambartian(COLOR_WHITE)));

    list.push(Object::with_material(cube, Glossy(1e3, 0.3, COLOR_WHITE)));

    let obj_arc = Arc::new(geom::GeometryList::from_vec(list));
    let mut integrator = integrator::WhittedIntegrator::new();
    integrator.antialiasing = 4;
    integrator.shadow_rays = 20;

    let pos = Vec3D::new(-6.0, -6.0, 3.0);
    let up = -Vec3D::z_axis();
    let focus = c + Vec3D::new(0.0, 0.0, 0.1); //Vec3D::new(-0.0, -0.0, 0.0);
    let cam = scene::Camera::new(1080, 960)
        .position(pos)
        .look_at(focus, up);

    let skybox = texture::Image::open("../data/skybox.jpg").unwrap();

    let lights: Vec<Box<dyn light::Light>> = vec![
        Box::new(AmbientLight::new(Vec3D::one(), 0.1)),
        Box::new(DirectionLight::new(
            vec3d(-2.5, -3.0, -3.0),
            0.01,
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


    {
        let i = std::env::args()
            .nth(1)
            .map(|x| x.parse::<u32>())
            .unwrap_or(Ok(5))
            .unwrap();

        let mut fast_integrator = integrator.clone();
        fast_integrator.shadow_rays = 1;
        fast_integrator.scatter_rays = 1;
        fast_integrator.antialiasing = i as i32;
        fast_integrator.max_depth = 5;
        println!("{:?}", fast_integrator);
        let img = render::parallel_render_image(&scene, &fast_integrator);
        img.save(&format!("test_{}.png", i)).unwrap();
    }
}
