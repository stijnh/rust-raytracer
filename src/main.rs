
extern crate image;
extern crate pbr;
extern crate rayon;
extern crate float_ord;
extern crate partition;
#[macro_use] extern crate derive_more;
#[macro_use] extern crate failure;

#[macro_use] mod util;
mod vec3;
mod world;
mod object;
mod loader;

use rayon::prelude::*;

use vec3::{vec3,Vec3};
use world::{Ray, Camera};
use object::{Sphere, Object, Cuboid, ObjectList, BoundingBox, Triangle, FastTriangle};
use float_ord::FloatOrd;
use partition::partition;
use std::sync::{Arc, Mutex};
use loader::load_obj;

fn divide_objects<T: 'static + Object + Clone>(objs: &mut [T], axis: u8, depth: u8) -> Box<dyn Object> {
    let n = objs.len();
    assert!(n > 0);

    if n == 1 {
        return Box::new(objs[0].clone());
    }

    if n < 5 || depth > 3 {
        return Box::new(ObjectList::new(objs.to_vec()));
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

        Box::new(ObjectList::new(vec![left, right]))
    }
}

fn create_world() -> Box<dyn Object> {
    let mut objs = vec![];
    let tri = load_obj("bunny.obj").unwrap();

    let transform = |v: Vec3| {
        vec3(v.z, v.x, v.y) * 1500.0 - vec3(10.0, -50.0, 100.0)
    };

    for (a, b, c) in tri.iter().cloned() {
        let a = transform(a);
        let b = transform(b);
        let c = transform(c);
        objs.push(FastTriangle::new(a, b, c));
    }

    let min_z = objs
        .iter()
        .map(|t| t.bounding_box().min.z)
        .map(|f| FloatOrd(f))
        .min()
        .unwrap().0;

    println!("tris={}", objs.len());

    let a = vec3(250.0, 250.0, min_z);
    let b = vec3(250.0, -250.0, min_z);
    let c = vec3(-250.0, 250.0, min_z);
    let d = vec3(-250.0, -250.0, min_z);
    objs.push(FastTriangle::new(a, c, b));
    objs.push(FastTriangle::new(c, d, b));

    /*
    c   a
    d   b
    */

    Box::new(divide_objects(&mut objs, 0, 0))

    /*
    for i in -100..=100 {
        for j in -100..=100 {
            for k in -100..=100 {
                let obj = Sphere::new(vec3(i as f32, j as f32, k as f32), 0.05);
                let c = vec3(i as f32, j as f32, k as f32);
                //let obj = Cuboid::new(c - 0.1, c + 0.1);
                //objs.push(Box::new(obj));
                //objs.push(obj);
            }
        }
    }


    let n = 10;
    let m = 10;

    let get_vertex = |i, j| {
        let radius = 50.0;
        let center = vec3(0.0, -50.0, 0.0)*0.0;

        let phi = (i as f32) / (n as f32) * std::f32::consts::PI;
        let rho = (j as f32 - i as f32*0.5) / (m as f32) * 2.0 * std::f32::consts::PI;

        let x = rho.cos() * phi.sin();
        let y = rho.sin() * phi.sin();
        let z = phi.cos();
       
        vec3(x, y, z) * radius + center
    };


    for i in 0..n {
        for j in 0..=m {
            let a = get_vertex(i, j);
            let b = get_vertex(i, j + 1);
            let c = get_vertex(i + 1, j);
            let d = get_vertex(i + 1, j + 1);

            objs.push(Triangle::new(a, b, d));
            objs.push(Triangle::new(a, c, d));
        }
    }

    let left = divide_objects(&mut objs, 0, 0);
    let right = Box::new(Sphere::new(vec3(0.0, 100.0, 0.0), 50.0));

    //Box::new(ObjectList::new(vec![left, right]))
    */
}

fn main() {

    type Pixel = image::Rgb<u8>;

    let subsampling = 2u32;
    let width = 800u32;
    let height = 600u32;
    let mut img = image::ImageBuffer::<Pixel, _>::new(width, height);

    let cam = Camera::new()
        //.position(vec3(125.0, -50.1, 20.0))
        //.look_at(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0))
        .position(vec3(200.0, 0.0, 100.0))
        .look_at(vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0))
        .perspective(100.0, width as f32, height as f32);

    let world = create_world();
    let light = vec3(0.0, 0.0, 1.0).normalize();

    {
        let mut bar = Mutex::new(pbr::ProgressBar::new((width * height * subsampling * subsampling) as u64));
        let mut pixels = vec![];

        (0..(width * height))
            .into_par_iter()
            .map(move |index| {
                let i = index % width;
                let j = index / width;
                let mut pixel = Vec3::zero();

                if i == 0 {
                    bar.lock().unwrap().add((width * subsampling * subsampling) as u64);
                }

                for a in 0..subsampling {
                    for b in 0..subsampling {
                        let x = (i as f32) + (a as f32 / subsampling as f32);
                        let y = (j as f32) + (b as f32 / subsampling as f32);
                        let ray = cam.ray_at(x, y);

                        let max_t = 10000.0;
                        let hit = world.hit(&ray, 0.0, max_t);

                        let p = if let Some(result) = hit {
                            let normal = result.normal.normalize();
                            let mut f = normal.dot(light).max(0.0);

                            let p = ray.pos + result.t * ray.dir;
                            let mut samples_total = 0;
                            let mut samples_hit = 0;

                            f *= {
                                let ray = Ray::new(p, vec3(0.0, 0.0, 1.0));
                                let hit = world.hit(&ray, 10.0, max_t).is_some();
                                if hit { 0.1 } else { 1.0 }
                            };

                            vec3(1.0, 1.0, 1.0) * f //* (1.0 - t / 100.0).min(1.0).max(0.0)
                        } else {
                            vec3(0.0, 0.0, 0.0)
                        };

                        pixel += p;
                    }
                }

                pixel *= 256.0 / (subsampling * subsampling) as f32;
                pixel = pixel.max(Vec3::fill(0.0));
                pixel = pixel.min(Vec3::fill(255.0));

                let data = [pixel.x as u8, pixel.y as u8, pixel.z as u8];
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
