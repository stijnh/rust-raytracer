
extern crate num;
extern crate image;
extern crate pbr;
extern crate rayon;
extern crate float_ord;
extern crate partition;
extern crate json;
#[macro_use] extern crate derive_more;
#[macro_use] extern crate failure;
#[macro_use] extern crate crunchy;

#[macro_use] mod util;
mod math;
mod camera;
mod loader;
mod geom;

use rayon::prelude::*;

use std::env;
use camera::Camera;
use geom::{Sphere, Geometry, GeometryList, BoundingBox, Triangle, Transform};
use float_ord::FloatOrd;
use partition::partition;
use std::sync::{Arc, Mutex};
use loader::{load_obj, load_scene};
use math::{vec3d, Ray, Vec3D, Dot, Quaternion};

fn divide_objects<T: 'static + Geometry + Clone>(objs: &mut [T], axis: u8, depth: u8) -> Box<dyn Geometry> {
    let n = objs.len();
    assert!(n > 0);

    if n == 1 {
        return Box::new(objs[0].clone());
    }

    if n < 5 || depth > 3 {
        return Box::new(GeometryList::from_vec(objs.to_vec()));
    }

    let mut centers: Vec<_> = objs.into_iter().map(|obj| {
        let bb = obj.bounding_box();
        let center = (bb.min + bb.max) / 2.0;
        center[axis as usize]
    }).collect();

    centers.sort_by_key(|f| FloatOrd(*f));
    let mid = centers[n / 2];

    let (before, after) = partition(objs, |obj| {
        let bb = obj.bounding_box();
        let center = (bb.min + bb.max) / 2.0;
        center[axis as usize] < mid
    });

    if before.len() == 0 {
        divide_objects(after, (axis + 1) % 3, depth + 1)
    } else if after.len() == 0 {
        divide_objects(before, (axis + 1) % 3, depth + 1)
    } else {
        let left = BoundingBox::new(divide_objects(before, (axis + 1) % 3, 0));
        let right = BoundingBox::new(divide_objects(after, (axis + 1) % 3, 0));

        Box::new(GeometryList::from_vec(vec![left, right]))
    }
}

fn create_world() -> Box<dyn Geometry> {
    let mut objs = vec![];
    let tri = load_obj("bunny.obj").unwrap();

    //let transform = |v: Vec3D| {
    //    vec3d(v[2], v[0], v[1]) * 1500.0 - vec3d(10.0, -50.0, 100.0)
    //};
    //

    for (a, b, c) in tri.iter().cloned() {
        objs.push(Triangle::new(a, b, c));
    }

    let min_y = objs
        .iter()
        .map(|t| t.bounding_box().min[1])
        .map(|f| FloatOrd(f))
        .min()
        .unwrap().0;

    println!("tris={}", objs.len());

    let a = vec3d(0.5, min_y, 0.5);
    let b = vec3d(-0.5, min_y, 0.5);
    let c = vec3d(0.5, min_y, -0.5);
    let d = vec3d(-0.5, min_y, -0.5);

    objs.push(Triangle::new(a, c, b));
    objs.push(Triangle::new(c, d, b));

    /*
    c   a
    d   b
    */

    let output = divide_objects(&mut objs, 0, 0);
    let output = Transform::new(output)
        .scale(14.0 * 2.50 * 100.0)
        .rotate_z(0.5 * 3.14)
        .rotate_x(0.5 * 3.14)
        .translate(vec3d(-30.0, 100.0, -300.0))
        ;
    println!("{:?}", output.bounding_box());
    Box::new(output)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let (cam, scene) = load_scene(&args[1]).unwrap();

    type Pixel = image::Rgb<u8>;

    let subsampling = 1u32;
    let width = 800u32;
    let height = 600u32;
    let mut img = image::ImageBuffer::<Pixel, _>::new(width, height);

    let cam = cam.perspective(100.0, width as f32, height as f32);

    let mut world = GeometryList::<Arc<dyn Geometry>>::new();
    world.add(create_world().into());
    let light = vec3d(0.0, 0.0, 1.0).normalize();


    {
        let mut bar = Mutex::new(pbr::ProgressBar::new((width * height * subsampling * subsampling) as u64));
        let mut pixels = vec![];

        (0..(width * height))
            .into_par_iter()
            .map(move |index| {
                let i = index % width;
                let j = index / width;
                let mut pixel = Vec3D::zero();

                if i == 0 {
                    bar.lock().unwrap().add((width * subsampling * subsampling) as u64);
                }

                for a in 0..subsampling {
                    for b in 0..subsampling {
                        let x = (i as f32) + (a as f32 / subsampling as f32);
                        let y = (j as f32) + (b as f32 / subsampling as f32);
                        let ray = cam.ray_at(x, y);

                        let max_t = 1e12;
                        let hit = world.hit(&ray, 0.0, max_t);

                        let p = if let Some(result) = hit {
                            let normal = result.norm.normalize();
                            let mut f = normal.dot(light).max(0.1);

                            let p = ray.pos + result.t * ray.dir;
                            let mut samples_total = 0;
                            let mut samples_hit = 0;

                            f *= {
                                let ray = Ray::new(p, vec3d(0.22, 0.22, 0.95));
                                let hit = world.hit(&ray, 0.1, max_t);
                                if hit.is_some() { 0.1 } else { 1.0 }
                            };

                            vec3d(1.0, 1.0, 1.0) * f //* (1.0 - t / 100.0).min(1.0).max(0.0)
                        } else {
                            vec3d(0.0, 0.0, 0.0)
                        };

                        pixel += p;
                    }
                }

                pixel *= 256.0 / (subsampling * subsampling) as f32;
                pixel = pixel.map(|v| max!(v, 0.0));
                pixel = pixel.map(|v| min!(v, 255.0));

                let data = [pixel[0] as u8, pixel[1] as u8, pixel[2] as u8];
                Pixel{data}
            })
            .collect_into_vec(&mut pixels);


            for (index, pixel) in pixels.into_iter().enumerate() {
                let i = (index as u32) % width;
                let j = (index as u32) / width;
                img.put_pixel(i, j, pixel);
            }
    }

    img.save("result.png").unwrap();
}
