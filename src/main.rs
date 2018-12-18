#![feature(const_fn)]

extern crate float_ord;
extern crate image;
extern crate json;
extern crate num;
extern crate partition;
extern crate pbr;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate crunchy;

#[macro_use]
mod util;
mod camera;
mod geom;
mod loader;
mod math;
mod texture;
mod scene;
mod material;
mod light;

use rayon::prelude::*;

use scene::Object;
use camera::Camera;
use float_ord::FloatOrd;
use geom::AABBTree;
use geom::{BoundingBox, Geometry, GeometryList, Sphere, Transform, Triangle, Translate, Cuboid};
use loader::{load_obj, load_scene};
use math::{vec3d, Dot, Quaternion, Ray, Vec3D};
use math::{cartesian_to_polar};
use partition::partition;
use std::env;
use std::sync::{Arc, Mutex};
use texture::{Texture, CheckerTexture, ImageTexture};
use material::{Material, ReflectionMaterial, LambertianMaterial};
use util::Color;
use light::Light;

use rand::{Rng, thread_rng};


fn create_world() -> impl Geometry {
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
        .unwrap()
        .0;

    println!("tris={}", objs.len());

    let a = vec3d(0.5, min_y, 0.5);
    let b = vec3d(-0.5, min_y, 0.5);
    let c = vec3d(0.5, min_y, -0.5);
    let d = vec3d(-0.5, min_y, -0.5);

    //objs.push(Triangle::new(a, c, b));
    //objs.push(Triangle::new(c, d, b));

    /*
    c   a
    d   b
    */

    //let output = divide_objects(&mut objs, 0, 0);
    
    //let obj = Sphere::new(Vec3D::zero(), 0.05);
    //let objs = vec![obj];

    let output = AABBTree::new(objs, 200.0);
    output.print_stats();

    let output = Transform::new(output)
        .scale(14.0 * 1.0 * 100.0)
        .rotate_z(0.5 * 3.14)
        .rotate_x(0.5 * 3.14)
        //.translate(vec3d(-30.0, 100.0, -300.0))
        ;
    println!("{:?}", output.bounding_box());

    let bunny: Arc<dyn Geometry> = Arc::new(output);
    let mut bunnies = vec![];

    for x in -1..=1 {
        for y in -1..=1 {
            let p = vec3d(-y as f32 * 400.0, x as f32 * 400.0, 0.0);

            bunnies.push(BoundingBox::new(Translate::new(bunny.clone(), p)));
        }
    }

    let objs = AABBTree::new(bunnies, 100.0);
    let bbox = objs.bounding_box();

    /*
    let a = vec3d(bbox.min[0], bbox.min[1], bbox.min[2]);
    let b = vec3d(bbox.max[0], bbox.min[1], bbox.min[2]);
    let c = vec3d(bbox.max[0], bbox.max[1], bbox.min[2]);
    let d = vec3d(bbox.min[0], bbox.max[1], bbox.min[2]);
    */

    let floor = Cuboid::new(
        bbox.min,
       vec3d(bbox.max[0], bbox.max[1], bbox.min[2] - 250.0));

    let checker = CheckerTexture::new(
        5,
        Color::one() * 0.9,
        Color::one() * 0.1);

    GeometryList::from_vec(vec![
                           Object::new(
                               Arc::new(objs) as Arc<dyn Geometry>,
                               Arc::new(ReflectionMaterial) as Arc<dyn Material>,
                            ),
                           Object::new(
                               Arc::new(floor) as Arc<dyn Geometry>,
                               Arc::new(LambertianMaterial::new(checker)) as Arc<dyn Material>,
                            ),
    ])



    //GeometryList::from_vec(bunnies)
}

fn random_in_sphere() -> Vec3D {
    let mut random = thread_rng();

    loop {
        let mut p = Vec3D::zero();

        for i in 0..3 {
            p[i] = random.gen::<f32>() * iff!(random.gen::<bool>(), -1.0, 1.0);
        }

        if p.dot(p) < 1.0 {
            break p;
        }
    }
}

fn illumunation(mut ray: Ray, world: &Geometry, lights: &[Light], skybox: &Texture) -> Color {
    let mut random = thread_rng();
    let mut pixel = Color::zero();
    let mut passthrough = Color::one();
    let mut bounces_left = 10;

    while !passthrough.is_zero() {
        let result = if bounces_left > 0 {
            bounces_left -= 1;
            world.hit(&ray, 0.0, 100000.0)
        } else {
            None
        };

        if let Some(hit) = result {
            let normal = hit.norm.normalize();
            let (l, r) = hit.material.get(hit.uv);
            let mut ill = Vec3D::zero();

            for light in lights {
                let mut rays_total = 0;
                let mut rays_hit = 0;
                let mut val = Vec3D::zero();

                loop {
                    let (r, t_min, t_max) = light.generate_ray(hit.pos, &mut random);
                    rays_total += 1;

                    if !world.is_hit(&r, t_min, t_max) {
                        val += r.dir.dot(normal).abs();
                        rays_hit += 1
                    }

                    if rays_total == 500 || (rays_total == 20 && (rays_hit == 0 || rays_hit == rays_total)) {
                        break;
                    }
                }

                ill += val / (rays_total as f32);
            }

            pixel += ill * passthrough * l;
            passthrough *= r;

            ray.dir -= 2.0 * ray.dir.dot(normal) * normal;
            ray.pos = hit.pos + ray.dir * 0.01;
        } else {
            let (s, c) = cartesian_to_polar(ray.dir);
            let (u, v) = (s / (2.0 * 3.14), c / 3.14);
            pixel += passthrough * skybox.color_at(u, v);
            break
        }
    }

    pixel
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let (cam, scene) = load_scene(&args[1]).unwrap();

    type Pixel = image::Rgb<u8>;

    let subsampling = 3u32;
    let width = 800u32 * 2;
    let height = 600u32 * 2;
    let shadow_minor_samples = 25;
    let shadow_major_samples = 250;
    let mut img = image::ImageBuffer::<Pixel, _>::new(width, height);

    let cam = cam.perspective(100.0, width as f32, height as f32);

    let texture: Arc<dyn Texture + Sync + Send> = Arc::new(Vec3D::one());
    //let texture = Arc::new(UVTexture);
    let skybox = Arc::new(ImageTexture::from_filename("skybox.jpg").unwrap());

    let mut world = GeometryList::new();
    world.add(create_world());

    let lights = vec![
        Light::new(Vec3D::new(0.0, 0.0, 1000.0), 100.0)
    ];


    {
        let mut bar = Mutex::new(pbr::ProgressBar::new(
            (width * height * subsampling * subsampling) as u64,
        ));
        let mut pixels = vec![];

        (0..(width * height))
            .into_par_iter()
            .map(move |index| {
                let i = index % width;
                let j = (height - 1) - index / width;
                let mut pixel = Vec3D::zero();

                if i == 0 {
                    bar.lock()
                        .unwrap()
                        .add((width * subsampling * subsampling) as u64);
                }

                if (i, j) != (200, 200) {
                    //return Pixel { data: [0, 0, 0] };
                }

                for a in 0..subsampling {
                    for b in 0..subsampling {
                        let x = (i as f32) + (a as f32 / subsampling as f32);
                        let y = (j as f32) + (b as f32 / subsampling as f32);
                        let mut ray = cam.ray_at(x, y);

                        pixel += illumunation(
                            ray,
                            &world,
                            &lights,
                            &*skybox);

                        /*
                        let max_t = 1e12;
                        let mut depth = 0;

                        let mut p = Vec3D::zero();
                        let mut thorough = Vec3D::one();

                        loop {
                            let hit = if depth < 5 {
                                depth += 1;
                                world.hit(&ray, 0.0, max_t)
                            } else {
                                None
                            };

                            if let Some(result) = hit {
                                let normal = result.norm.normalize();
                                let mut shadow_hits = 0;
                                let mut shadow_tries = 0;

                                for _ in 0..shadow_major_samples {
                                    if shadow_tries == shadow_minor_samples {
                                        if shadow_hits == 0 || shadow_hits == shadow_tries {
                                            break;
                                        }
                                    }

                                    let shadow_dir = (light + random_in_sphere() * 0.5).normalize();
                                    let shadow_ray = Ray::new(result.pos, shadow_dir);
                                    
                                    if !world.is_hit(&shadow_ray, 0.01, max_t) {
                                        shadow_hits += 1;
                                    }

                                    shadow_tries += 1;
                                }

                                let light_frac = shadow_hits as f32 / shadow_tries as f32;

                                if true || normal.dot(Vec3D::unit_z()).abs() < 0.99 {
                                    p += thorough * 0.5 * normal.dot(light).abs() * light_frac;
                                    thorough *= 0.5;
                                } else {
                                    p += thorough * normal.dot(light).abs() * light_frac;
                                    thorough *= 0.0;
                                }
                                
                                let d = ray.dir - 2.0 * ray.dir.dot(normal) * normal;
                                let real_d = (50.0 * d + 0.0*random_in_sphere()).normalize();

                                let p = ray.pos + result.t * ray.dir + real_d * 0.01;

                                ray = Ray::new(p, real_d)
                            } else {
                                break
                            }
                        };
                        */

                    }
                }

                pixel *= 256.0 / (subsampling * subsampling) as f32;
                pixel = pixel.map(|v| max!(v, 0.0));
                pixel = pixel.map(|v| min!(v, 255.0));

                let data = [pixel[0] as u8, pixel[1] as u8, pixel[2] as u8];
                Pixel { data }
            }).collect_into_vec(&mut pixels);

        for (index, pixel) in pixels.into_iter().enumerate() {
            let i = (index as u32) % width;
            let j = (index as u32) / width;
            img.put_pixel(i, j, pixel);
        }
    }

    img.save("result.png").unwrap();
}
