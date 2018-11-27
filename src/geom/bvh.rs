use math::{Vec3D, AABB, Ray};
use geom::{Geometry, HitResult};
use std::f32;
use rand::seq::SliceRandom;

pub struct AABBTree<T> {
    objs: Box<[T]>,
    nodes: Box<[(AABB, u32, u32)]>,
}

pub fn partition_bin_sah<T>(objs: &mut [T]) -> Option<usize> 
        where T: Geometry {
    if objs.len() < 7 {
        return None;
    }

    let surface = |bbox: AABB| {
        let delta = bbox.max - bbox.min;
        delta[0] * delta[1] + delta[1] * delta[2] + delta[2] * delta[0]
    };

    let bbox = objs
        .iter()
        .map(|obj| obj.bounding_box())
        .collect::<Vec<_>>();

    let mut centers = bbox
        .iter()
        .map(|b| (b.min + b.max) / 2.0)
        .collect::<Vec<_>>();

    let mut best = (f32::INFINITY, 0, 0.0);

    for axis in 0..3 {
        let (mut min, mut max) = (f32::INFINITY, -f32::INFINITY);

        for c in &centers {
            min = min!(min, c[axis]);
            max = max!(max, c[axis]);
        }

        const NUM_BUCKETS: usize = 8;
        let mut bucket_sizes = [0; NUM_BUCKETS];
        let mut bucket_bbox = [AABB::empty(); NUM_BUCKETS];

        for (c, b) in centers.iter().zip(bbox.iter()) {
            let ratio = (c[axis] - min) / (max - min);
            let index = ((ratio * NUM_BUCKETS as f32).floor() as usize).min(NUM_BUCKETS - 1);

            bucket_sizes[index] += 1;
            bucket_bbox[index] = bucket_bbox[index].union(b);
        }

        for i in 1..NUM_BUCKETS {
            let lsize: usize = bucket_sizes[..i].iter().sum();
            let rsize: usize = bucket_sizes[i..].iter().sum();

            if lsize == 0 || rsize == 0 { continue; }

            let lbox = bucket_bbox[..i].iter().fold(AABB::empty(), |a, b| a.union(b));
            let rbox = bucket_bbox[i..].iter().fold(AABB::empty(), |a, b| a.union(b));

            let cost: f32 = lsize as f32 * surface(lbox) + rsize as f32 * surface(rbox);

            if cost < best.0 {
                let mid = (i as f32 / NUM_BUCKETS as f32) * (max - min) + min;
                best = (cost, axis, mid);
            }
        }
    }

    let n = objs.len();
    let (mut i, mut j) = (0, n);
    let (_, best_axis, best_mid) = best;

    loop { 
        while i < n && centers[i][best_axis] >= best_mid {
            i += 1;
        }

        while j > 0 && centers[j - 1][best_axis] < best_mid { 
            j -= 1;
        }

        if i + 1 < j {
            centers.swap(i, j - 1);
            objs.swap(i, j - 1);
        } else {
            break;
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
        Self::with_partition(objs, &partition_bin_sah)
    }
}

impl <T: Geometry>  Geometry for AABBTree<T> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut i = 0;
        let n = self.nodes.len() - 1;
        let mut result = None;
        let mut visited = 0;
        let mut visitez = 0;

        while i < n {
            let node = &self.nodes[i];
            visitez += 1;

            if let Some((t0, t1)) = node.0.intersect_ray(ray) {
                if t0 - 0.01 <= t_max && t1 + 0.01  >= t_min {
                    let begin = node.2 as usize;
                    let end = self.nodes[i + 1].2 as usize;

                    for obj in &self.objs[begin..end] {
                        if let Some(r) = obj.hit(ray, t_min, t_max) {
                            t_max = r.t;
                            result = Some(r);
                        }

                        visited += 1;
                    }

                    i += 1;
                } else {
                    i = node.1 as usize;
                }
            } else {
                i = node.1 as usize;
            }
        }

        if visited > 0 && self.objs.len() > 2000 {
            println!("{}/{} = {}, {}/{} = {}",
                     visited, self.objs.len(),
                     visited as f32/self.objs.len() as f32,
                     visitez, self.nodes.len(),
                     visitez as f32 / self.nodes.len() as f32,
                     
                     );
        }

        result
    }

    fn bounding_box(&self) -> AABB {
        self.nodes[0].0
    }
}
