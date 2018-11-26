use math::{Vec3D, AABB, Ray};
use geom::{Geometry, HitResult};
use std::f32;
use rand::seq::SliceRandom;

pub struct AABBTree<T> {
    objs: Box<[T]>,
    nodes: Box<[(AABB, u32, u32)]>,
}

pub fn partition_random<T>(objs: &mut [T]) -> Option<usize>
        where T: Geometry {
    if objs.len() < 5 {
        return None;
    }

    Some(objs.len() / 2)
}

pub fn partition_sah<T>(objs: &mut [T]) -> Option<usize> 
        where T: Geometry {
    if objs.len() < 5 {
        return None;
    }

    let bbox = objs
        .iter()
        .map(|obj| obj.bounding_box())
        .collect::<Vec<_>>();

    let n = objs.len();
    let mut rng = ::rand::thread_rng();
    let mut indices = (0..n).collect::<Vec<_>>();
    let mut best = (f32::INFINITY, vec![]);

    let surface = |bbox: AABB| {
        let delta = bbox.max - bbox.min;
        delta[0] * delta[1] + delta[1] * delta[2] + delta[2] * delta[0]
    };

    for tries in 0..25 {
        let mut membership = vec![];
        membership.resize(n, false);

        indices.shuffle(&mut rng);
        let mut lbox = bbox[indices[0]];
        let mut rbox = bbox[indices[1]];
        membership[indices[1]] = true;

        let mut lsize = 1;
        let mut rsize = 1;

        let mut lcost = surface(lbox);
        let mut rcost = surface(rbox);

        for i in indices[2..].iter().cloned() {
            let new_lbox = lbox.union(&bbox[i]);
            let new_rbox = rbox.union(&bbox[i]);

            let new_lcost = (lsize + 1) as f32 * surface(new_lbox);
            let new_rcost = (rsize + 1) as f32 * surface(new_rbox);

            if new_lcost + rcost < lcost + new_rcost {
                lbox = new_lbox;
                lcost = new_lcost;
                lsize += 1;
            } else {
                membership[i] = true;
                rbox = new_rbox;
                rcost = new_rcost;
                rsize += 1;
            }
        }

        let cost = lcost + rcost;
        if n > 100 {
            println!("{} + {} = {}\t{}", 
                     lsize, rsize, n, cost);
         }

        if cost < best.0 {
            best = (cost, membership);
        }
    }

    let mut membership = best.1;
    let (mut i, mut j) = (0, objs.len() - 1);

    loop {
        while i < n && membership[i] {
            i += 1;
        }

        while j >= 0 && !membership[j] {
            j -= 1;
        }

        if i < j {
            membership.swap(i, j);
            objs.swap(i, j);
        } else {
            break;
        }
    }

    Some(i)
}

pub fn partition_average<T>(objs: &mut [T]) -> Option<usize> 
        where T: Geometry {
    if objs.len() < 5 {
        return None;
    }

    let mut centers = objs
        .iter()
        .map(|obj| obj.bounding_box())
        .map(|b| (b.max + b.min) / 2.0)
        .collect::<Vec<_>>();

    let mut min = Vec3D::fill(f32::INFINITY);
    let mut max = Vec3D::fill(-f32::INFINITY);

    for center in &centers {
        min = Vec3D::from_map(|i| min!(min[i], center[i]));
        max = Vec3D::from_map(|i| max!(max[i], center[i]));
    }

    let diff = max - min;
    let best_axis = 
        if diff[0] > diff[1] && diff[0] > diff[2] { 0 }
        else if diff[1] > diff[2] { 1 }
        else { 2 };

    let mid = (min +  max)[best_axis] / 2.0;

    let (mut i, mut j) = (0, objs.len() - 1);

    while i < j {
        if centers[i][best_axis] > mid {
            centers.swap(i, j);
            objs.swap(i, j);

            j -= 1;
        } else {
            i += 1;
        }
    }

    Some(i)
}

impl<T: Geometry> AABBTree<T> {
    fn construct_node<F>(objs: &mut Vec<T>, begin: usize, end: usize, 
            nodes: &mut Vec<(AABB, u32, u32)>, partition: &F)
            where F: Fn(&mut[T]) -> Option<usize> {
        let index = nodes.len();
        nodes.push((AABB::empty(), 0, 0));

        if let Some(len) = partition(&mut objs[begin..end]) {
            if len > 0 && begin + len < end {
                Self::construct_node(objs, begin, begin + len, nodes, partition);
                Self::construct_node(objs, begin + len, end, nodes, partition);
            }
        }

        let bbox = objs[begin..end]
            .iter()
            .fold(AABB::empty(), |a, b| a.union(&b.bounding_box()));

        nodes[index] = (bbox, nodes.len() as u32, begin as u32);
    }

    pub fn with_partition<F>(mut objs: Vec<T>, partition: &F) -> Self 
            where F: Fn(&mut[T]) -> Option<usize> {
        let mut nodes = Vec::new();
        let n = objs.len();
        Self::construct_node(&mut objs, 0, n, &mut nodes, partition);

        let m = nodes.len();
        nodes.push((AABB::empty(), m as u32, n as u32));

        AABBTree {
            objs: objs.into_boxed_slice(),
            nodes: nodes.into_boxed_slice(),
        }
    }

    pub fn new(objs: Vec<T>) -> Self {
        Self::with_partition(objs, &partition_sah)
    }
}

impl <T: Geometry>  Geometry for AABBTree<T> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut i = 0;
        let n = self.nodes.len() - 1;
        let mut result = None;

        while i < n {
            let node = &self.nodes[i];

            if let Some((t0, t1)) = node.0.intersect_ray(ray) {
                if t0 - 0.01 <= t_max && t1 + 0.01  >= t_min {
                    let begin = node.2 as usize;
                    let end = self.nodes[i + 1].2 as usize;

                    for obj in &self.objs[begin..end] {
                        if let Some(r) = obj.hit(ray, t_min, t_max) {
                            t_max = r.t;
                            result = Some(r);
                        }
                    }

                    i += 1;
                } else {
                    i = node.1 as usize;
                }
            } else {
                i = node.1 as usize;
            }
        }

        result
    }

    fn bounding_box(&self) -> AABB {
        self.nodes[0].0
    }
}
