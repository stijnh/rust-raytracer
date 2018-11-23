use object::{AABB, Object, HitResult, ObjectList};
use world::Ray;
use util::Vec3D;
use std::f32::INFINITY;

/*
pub enum Tree<T: Object> {
    Inner(Box<[Tree<T>]>),
    Leaf(Box<[T]>),
}

fn divide_round_robin_mean<T: Object>(objs: Vec<T>, threshold: usize) {
    fn divide<T: Object>(objs: Vec<T>, axis: usize, tries: usize) {
        if objs.len() < threshold || tries > 3 {
            return Tree::Leaf(objs.into_boxed_slice());
        }

        let centers = objs.iter()
            .map(|o| o.bounding_box())
            .map(|b| (b.min[axis] + b.max[axis]) / 2.0)
            .collect::<Vec<_>>();
        let mean = centers.sum() / centers.len();
        let lhs = objs.drain_filter();
        let rhs = ();

        if lhs.len() == 0 {
            divide(rhs, (axis + 1) % 3, tries + 1)
        } else if rhs.len() == 0 {
            divide(lhs, (axis + 1) % 3, tries + 1)
        } else {
            let a = divide(lhs, (axis + 1) % 3, tries + 1);
            let b = divide(rhs, (axis + 1) % 3, tries + 1);
            Tree::Inner(vec![a, b].into_boxed_slice())
        }
    };

    divide(objs, 0, 0);
}


fn calculate_bbox<F, I>(list: I, fun: F) -> Option<AABB>
        where I: Iterator, F: Fn(I::Item) -> AABB {
    let mut bbox: Option<AABB> = None;

    for item in list {
        let b = fun(item);

        bbox = Some(match bbox {
            Some(a) => a.union(&b),
            None => b,
        });
    }

    bbox
}

enum AABBTree<T: Object> {
    Inner(AABB, Box<[AABBTree<T>]>),
    Leaf(AABB, ObjectList<T>),
}

impl <T: Object> AABBTree<T> {
    pub fn from_tree(tree: Tree<T>) -> Self {
        match tree {
            Tree::Leaf(nodes) => {
                let list = ObjectList::new(nodes.into_vec());
                let mut bbox = list.bounding_box();

                bbox.min -= Vec3D::from_scalar(0.01);
                bbox.max += Vec3D::from_scalar(0.01);

                AABBTree::Leaf(bbox, list)
            },

            Tree::Inner(nodes) => {
                let children: Vec<_> = nodes.into_vec().into_iter().map(AABBTree::from_tree).collect();
                let bbox = calculate_bbox(children.iter(), Object::bounding_box);

                AABBTree::Inner(bbox.unwrap(), children.into_boxed_slice())
            }
        }
    }
}

impl <T: Object> Object for AABBTree<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let (t_in, t_out) = self.bounding_box().intersect_ray(ray)?;
        let (t0, mut t1)= (max!(t_in, t_min), min!(t_out, t_max));

        if t0 > t1 {
            return None;
        }

        match self {
            AABBTree::Leaf(_, list) => list.hit(ray, t0, t1),
            AABBTree::Inner(_, children) => {
                let mut out = None;

                for child in children.iter() {
                    if let Some(result) = child.hit(ray, t0, t1) {
                        t1 = result.t;
                        out = Some(result)
                    }
                }

                out
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        *match self {
            AABBTree::Leaf(x, _) => x,
            AABBTree::Inner(x, _) => x,
        }
    }
}

union KDTreeNode {
    inner: (u32, f32, f32),
    leaf: (u32, u32, u32),
}

pub struct KDTree<T> {
    nodes: Box<[KDTreeNode]>,
    objects: Box<[T]>,
    max_depth: u8,
}

impl <T: Object> KDTree<T> {
    fn create_node(tree: Tree<T>, depth: usize, max_depth: &mut usize, 
                   nodes: &mut Vec<KDTreeNode>, objects: &mut Vec<T>, parent_bbox: AABB) {
        fn tree_bbox<T: Object>(tree: &Tree<T>) -> AABB {
            match tree {
                Tree::Inner(children) => calculate_bbox(children.iter(), tree_bbox),
                Tree::Leaf(children) => calculate_bbox(children.iter(), Object::bounding_box),
            }.unwrap()
        }
        let bbox = tree_bbox(&tree);

        match tree {
            Tree::Inner(children) => {
                *max_depth = max!(depth, *max_depth);

                let index = nodes.len();
                nodes.push(KDTreeNode{ inner: (0, 0.0, 0.0) });

                let fx = (bbox.max[0] - bbox.min[0]) / (parent_bbox.max[0] - parent_bbox.min[0]);
                let fy = (bbox.max[1] - bbox.min[1]) / (parent_bbox.max[1] - parent_bbox.min[1]);
                let fz = (bbox.max[2] - bbox.min[2]) / (parent_bbox.max[2] - parent_bbox.min[2]);

                let axis = if fx < fy && fx < fz {
                    0
                } else if fy < fz {
                    1
                } else {
                    2
                };

                let min = bbox.min[axis] - 0.001;
                let max = bbox.max[axis] + 0.001;

                let mut my_bbox = parent_bbox.clone();
                my_bbox.min[axis] = min;
                my_bbox.max[axis] = max;

                for node in children.into_vec() {
                    Self::create_node(node, depth + 1, max_depth, nodes, objects, my_bbox);
                }

                let skip = nodes.len() - index;

                assert!(axis < 3);        // 2 bit
                assert!(depth < 64);      // 6 bit
                assert!(skip < 16777216); // 24 bit

                let tag = ((axis << 30) | (depth << 24) | skip) as u32;
                nodes[index] = KDTreeNode { inner: (tag, min, max) };
            },

            Tree::Leaf(children) => {
                let begin = objects.len();
                objects.extend(children.into_vec());
                let end = objects.len();

                assert!(end < (i32::max_value() as usize));

                nodes.push(KDTreeNode { leaf: (!0, begin as u32, end as u32) });
            }
        }
    }

    pub fn from_tree(tree: Tree<T>) -> Self {
        let mut nodes = vec![];
        let mut objects = vec![];
        let mut max_depth = 0;
        let bbox = AABB::new(Vec3D::from_scalar(-INFINITY), Vec3D::from_scalar(INFINITY));
        Self::create_node(tree, 0, &mut max_depth, &mut nodes, &mut objects, bbox);

        KDTree {
            nodes: nodes.into_boxed_slice(), 
            objects: objects.into_boxed_slice(), 
            max_depth: !0
        }
    }
}

impl <T:Object> Object for KDTree<T> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitResult> {
        let mut out = None;
        let mut index = 0;
        let length = self.nodes.len();

        let mut ranges = vec![(0.0, 0.0); self.max_depth as usize];
        ranges[0] = (t_min, t_max);

        while index < length {
            let node = &self.nodes[index];
            let tag = unsafe { node.inner.0 };

            let skip = (tag & 0x00ffffff) as usize;
            let depth = ((tag & 0x3f000000) >> 24) as usize;
            let axis = ((tag & 0xc0000000) >> 30) as usize;
            let (t_in, t_out) = ranges[depth as usize];

            if axis < 3 {
                let (_, begin, end) = unsafe { node.inner };

                let x = ray.pos[axis];
                let dx = ray.dir[axis];
                let t0 = (x - begin) * (1.0 / dx);
                let t1 = (x - end) * (1.0 / dx);

                let t_in = max!(min!(t0, t1), t_in, t_min);
                let t_out = min!(max!(t0, t1), t_out, t_max);

                if t_in < t_out {
                    ranges[depth + 1] = (t_in, t_out);
                    index += 1;
                } else {
                    index += skip;
                }
            } else {
                let (_, begin, end) = unsafe { node.leaf };
                let (begin, end) = (begin as usize, end as usize);

                for obj in &self.objects[begin..end] {
                    if let Some(result) = obj.hit(ray, t_in, t_out.min(t_max)) {
                        t_max = result.t;
                        out = Some(result);
                    }
                }

                index += 1;
            }
        }

        out
    }

    fn bounding_box(&self) -> AABB {
        calculate_bbox(self.objects.iter(), Object::bounding_box).unwrap()
    }
}

*/
