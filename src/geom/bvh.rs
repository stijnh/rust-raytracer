use math::{Vec3, AABB, Ray};

struct AABBTree<T> {
    objs: Box<[T]>,
    nodes: Box<[(AABB, u32, u32)]>,
};

pub fn partition_median<T>(objs: &mut [T]) -> Option<usize> 
        where T: Geometry {
    if objs.len() < 5 {
        return None;
    }

    let centers = objs
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
        if centers[i] > mid {
            swap(centers[i], centers[j]);
            swap(objs[i], objs[j]);

            j -= 1;
        } else {
            i += 1;
        }
    }

    if i > 0 && i + 1 < objs.len() {
        Some(i)
    } else {
        None
    }
}

impl<T: Geometry> AABBTree<T> {
    fn construct_node<F>(objs: &mut Vec<T>, begin: usize, end: usize, 
            nodes: &mut Vec<(AABB, u32, u32)>, partition: F)
            where F: Fn(&mut[T]) -> Option<usize> {
        let index = nodes.len();
        nodes.push((AABB::empty(), 0, 0));

        if let Some(len) = partition(objs[begin..end]) {
            construct_node(objs, begin, begin + len, nodes, partition);
            construct_node(objs, begin + len, end, nodes, partition);
        }

        let bbox = objs[begin..end]
            .iter()
            .fold(AABB::empty(), |a, b| a.union(b.bounding_box()));

        nodes[index] = (bbox, nodes.len(), begin);
    }

    pub fn with_partition(objs: Vec<T>, partition: F) -> Self 
            where F: Fn(&mut[T]) -> Option<usize> {
        let nodes = Vec::new();
        Self::construct_node(&mut obj, 0, objs.len(), &mut nodes, partition);

        nodes.push((AABB::empty(), objs.len(), nodes.len()));

        AABB {
            objs: objs.into_sliced_box(),
            nodes: nodes.into_sliced_box(),
        }
    }

    pub fn new(objs) -> Self {
        Self::with_partition(partition_median);
    }
}

impl <T: Geometry>  Geometry for AABBTree<T> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut i = 0;
        let n = len(self.nodes) - 1;
        let result = None;

        while i < n {
            let node = &self.nodes[i];

            if let Some((t0, t1)) = node.0.intersect_ray(ray) {
                if t0 <= t_max && t1  >= t_min {
                    let begin = node.2 as usize;
                    let end = (nodes[i + 1].2 & mask) as usize;

                    for obj in self.objs[begin..end] {
                        if Some(r) = obj.hit(ray, t_min, t_max) {
                            t_max = r.t;
                            result = Some(r);
                        }
                    }

                    i += 1;
                } else {
                    i = node.1;
                }
            } else {
                i = node.1;
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.nodes[0].0
    }
}
