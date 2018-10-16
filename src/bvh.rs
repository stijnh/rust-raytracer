use object::{AABB, Object, HitResult, ObjectList};
use world::Ray;

/*
struct AABBTree<T: Object> {
    bbox: AABB,
    children: enum {
        Leaf(ObjectList<T>),
        Node(Box[AABBTree; 2]),
    }
}

impl Hit for AABBTree {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut stack = vec![];
        let mut out = None;

        stack.push((self, t_min, t_max));

        while let Some((node, mut t0, mut t1)) = stack.pop() {
            if t0 > t1 || t0 > t_max || t1 < t_min {
                continue;
            }

            if let Some(t_in, t_out) = node.bbox.ray_intersect(ray) {
                if t0 < t_out && t1 > t_in {
                    t0 = max!(t0, t_in);
                    t1 = min!(t1, t_out, t_max);
                } else {
                    continue;
                }
            } else {
                continue;
            }

            match node.children {
                AABBTree::Leaf(list) => {
                    if let Some(result) = list.hit(t0, t1) {
                        t_max = result.t;
                        out = result;
                    }
                },

                AABBTree::Node([lhs, rhs]) => {
                    stack.push((lhs, t0, t1));
                    stack.push((rhs, t0, t1));
                }
            }
        }

        out
    }

    fn bounding_box(&self) -> AABB {
        match self {
            AABBTree::Leaf(list) => {
                list.iter().fold(None, |bbox, obj| => {
                    let x = obj.bounding_box();
                    let b = bbox.unwrap_or(x);
                    b.min = x.min.min(b.min);
                    b.max = x.max.max(b.max);
                    Some(bbox)
                }).unwrap()
            },
            AABBTree::Node(bbox, _) {
                *bbox
            }
        }
    }
}

struct KDTreeNode {
    depth: u8,
    axis: u8,
    side: i8,
    split: f32,
    skip: u32,
    object_offset: u32,
}

pub struct KDTree<T> {
    nodes: Box<[KDTreeNode]>,
    objects: Box<[T]>,
    max_depth: u8,
}

impl Object for KDTree<T: Object> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut out = None;
        let mut index = 0;
        let length = self.nodes.len();

        let mut ranges = vec![(0.0, 0.0); self.max_depth].into_boxed_slice();
        ranges[0] = (t_min, t_max);

        while index < length - 1 {
            let node = self.nodes[index];
            let depth = node.depth;
            let (mut t0, mut t1) = ranges[depth as usize];
            t1 = min!(t1, t_max);

            let x = ray.pos[node.axis as usize];
            let dx = ray.dir[node.axis as usize];
            let t = (node.split - x) / dx;

            if dx.abs() > 0.01 {
                if (dx < 0) ^ (side == +1) {
                    t0 = max!(t, t0);
                } else {
                    t1 = min!(t, t1);
                }
            }

            if t0 > t1 {
                index = node.skip;
                continue;
            }

            let begin = node.object_offset as usize;
            let end = self.nodes[index + 1].object_offset as usize;

            for obj in self.objects[begin..end] {
                if let Some(result) = obj.hit(t0, t1) {
                    t_max = result.t;
                    t1 = result.t;
                    out = Some(result);
                }
            }

            ranges[(depth + 1) as usize] = (t0, t1);
            index += 1;
        }

        out
    }

    fn bounding_box(&self) -> AABB {
        let mut bbox = self.objects[0].bounding_box();

        for obj in self.objects {
            let x = obj.bounding_box();
            bbox.min = bbox.min.min(x.min);
            bbox.max = bbox.max.max(x.max);
        }

        bbox
    }
}

*/
