use crate::geom::{Geometry, HitResult};
use crate::math::*;
use std::f32;

pub struct AABBTree<T> {
    objs: Box<[T]>,
    nodes: Box<[(AABB, u32, u32)]>,
}

pub fn partition_sah<T>(bbox: &mut [AABB], objs: &mut [T]) -> Option<usize>
where
    T: Geometry,
{
    let surface = |bbox: AABB| {
        let delta = bbox.max - bbox.min;
        delta[0] * delta[1] + delta[1] * delta[2] + delta[2] * delta[0]
    };

    let mut centers = bbox
        .iter()
        .map(|b| (b.min + b.max) * 0.5)
        .collect::<Vec<_>>();

    let mut best = (f32::INFINITY, 0, 0.0);

    for axis in 0..3 {
        let (mut min, mut max) = (f32::INFINITY, -f32::INFINITY);

        for c in &centers {
            min = min!(min, c[axis]);
            max = max!(max, c[axis]);
        }

        const NUM_BUCKETS: usize = 16;
        let mut bucket_sizes = [0; NUM_BUCKETS];
        let mut bucket_bbox = [AABB::new(); NUM_BUCKETS];

        for (c, b) in centers.iter().zip(bbox.iter()) {
            let ratio = (c[axis] - min) / (max - min);
            let index = ((ratio * NUM_BUCKETS as f32).floor() as usize).min(NUM_BUCKETS - 1);

            bucket_sizes[index] += 1;
            bucket_bbox[index] = AABB::union(bucket_bbox[index], *b);
        }

        for i in 1..NUM_BUCKETS {
            let lsize: usize = bucket_sizes[..i].iter().sum();
            let rsize: usize = bucket_sizes[i..].iter().sum();

            if lsize == 0 || rsize == 0 {
                continue;
            }

            let mut lbox = AABB::new();
            let mut rbox = AABB::new();

            for &b in &bucket_bbox[..i] {
                lbox = lbox.union(b);
            }

            for &b in &bucket_bbox[i..] {
                rbox = rbox.union(b);
            }

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
            bbox.swap(i, j - 1);
            centers.swap(i, j - 1);
            objs.swap(i, j - 1);
        } else {
            break;
        }
    }

    Some(i)
}

impl<T: Geometry> AABBTree<T> {
    fn construct_node<F>(
        bbox: &mut [AABB],
        objs: &mut [T],
        begin: usize,
        end: usize,
        nodes: &mut Vec<(AABB, u32, u32)>,
        partition: &F,
        hit_cost: f32,
    ) where
        F: Fn(&mut [AABB], &mut [T]) -> Option<usize>,
    {
        let bb = bbox[begin..end]
            .iter()
            .fold(AABB::new(), |a, &b| a.union(b));

        let index = nodes.len();
        nodes.push((AABB::new(), !0 as u32, !0 as u32));

        if let Some(len) = partition(&mut bbox[begin..end], &mut objs[begin..end]) {
            let mid = begin + len;
            let lsize = mid - begin;
            let rsize = end - mid;

            if lsize > 0 && rsize > 0 {
                let lsurface = bbox[begin..mid]
                    .iter()
                    .fold(AABB::new(), |a, &b| a.union(b))
                    .surface_area();

                let rsurface = bbox[begin..mid]
                    .iter()
                    .fold(AABB::new(), |a, &b| a.union(b))
                    .surface_area();

                let surface = bb.surface_area();

                let split_cost = 1.0
                    + hit_cost * (lsurface / surface) * (lsize as f32)
                    + hit_cost * (rsurface / surface) * (rsize as f32);
                let nosplit_cost = hit_cost * (lsize + rsize) as f32;

                if split_cost < nosplit_cost {
                    Self::construct_node(bbox, objs, begin, mid, nodes, partition, hit_cost);
                    Self::construct_node(bbox, objs, mid, end, nodes, partition, hit_cost);
                }
            }
        }

        nodes[index] = (bb, nodes.len() as u32, begin as u32);
    }

    pub fn with_partition<F>(mut objs: Vec<T>, partition: &F, hit_cost: f32) -> Self
    where
        F: Fn(&mut [AABB], &mut [T]) -> Option<usize>,
    {
        let n = objs.len();
        let mut nodes = Vec::new();
        let mut bbox = objs
            .iter()
            .map(|obj| obj.bounding_box())
            .collect::<Vec<_>>();

        Self::construct_node(&mut bbox, &mut objs, 0, n, &mut nodes, partition, hit_cost);

        let m = nodes.len();
        nodes.push((AABB::new(), m as u32, n as u32));

        AABBTree {
            objs: objs.into_boxed_slice(),
            nodes: nodes.into_boxed_slice(),
        }
    }

    pub fn new(objs: Vec<T>, hit_cost: f32) -> Self {
        let out = Self::with_partition(objs, &partition_sah, hit_cost);
        out.print_stats();
        out
    }

    pub fn print_stats(&self) {
        let n = self.nodes.len();
        let mut max_leaf = 0;
        let mut min_leaf = self.objs.len() as u32;
        let mut num_leaf = 0;

        for i in 0..n - 1 {
            let size = self.nodes[i + 1].2 - self.nodes[i].2;

            if size > 0 {
                min_leaf = min!(min_leaf, size);
                max_leaf = max!(max_leaf, size);
                num_leaf += 1;
            }
        }

        println!(
            "BVH statistics: {} objs, {} nodes, {} leafs, min/max/avg leaf: {}/{}/{}",
            self.objs.len(),
            self.nodes.len(),
            num_leaf,
            min_leaf,
            max_leaf,
            (self.objs.len() as f32) / (num_leaf as f32)
        );
    }

    #[inline(always)]
    fn traverse<'a, F, R>(
        &'a self,
        ray: &Ray,
        mut t_max: f32,
        exit_immediate: bool,
        fun: F,
    ) -> Option<R>
    where
        F: Fn(&'a T, &Ray, &mut f32) -> Option<R>,
    {
        let mut i = 0;
        let n = self.nodes.len() - 1;
        let mut result = None;
        let ray_inv_dir = 1.0 / ray.dir;
        let ray_neg_dir = [
            ray_inv_dir[0].is_sign_negative(),
            ray_inv_dir[1].is_sign_negative(),
            ray_inv_dir[2].is_sign_negative(),
        ];

        while i < n {
            let node = &self.nodes[i];

            if let Some((t0, t1)) = node.0.fast_intersect_ray(ray.pos, ray_inv_dir, ray_neg_dir) {
                if t0 <= t_max + 0.01 && t1 >= -0.01 {
                    let begin = node.2 as usize;
                    let end = self.nodes[i + 1].2 as usize;

                    for obj in &self.objs[begin..end] {
                        if let Some(r) = fun(obj, ray, &mut t_max) {
                            result = Some(r);

                            if exit_immediate {
                                return result;
                            }
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
}

impl<T: Geometry> Geometry for AABBTree<T> {
    #[inline(never)]
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        self.traverse(ray, t_max, false, |obj, ray, t_max| {
            if let Some(hit) = obj.hit(ray, *t_max) {
                *t_max = hit.t;
                Some(hit)
            } else {
                None
            }
        })
    }

    #[inline(never)]
    fn is_hit(&self, ray: &Ray, t_max: f32) -> bool {
        self.traverse(ray, t_max, true, |obj, ray, t_max| {
            if obj.is_hit(ray, *t_max) {
                Some(())
            } else {
                None
            }
        })
        .is_some()
    }

    fn bounding_box(&self) -> AABB {
        self.nodes[0].0
    }
}
