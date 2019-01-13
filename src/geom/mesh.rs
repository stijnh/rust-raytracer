use crate::geom::triangle::moller_trumbore;
use crate::geom::{AABBTree, Geometry, HitResult, Triangle};
use crate::material::DEFAULT_MATERIAL;
use crate::math::*;
use crunchy::unroll;
use delegate::*;
use std::collections::HashMap;
use std::mem::transmute;
use std::sync::Arc;

struct MeshTriangle {
    vertices: [u32; 3],
    data: Arc<[Vec3D]>,
}

pub struct Mesh {
    tree: AABBTree<MeshTriangle>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3D>, normals: Vec<Vec3D>, faces: Vec<[u32; 3]>) -> Self {
        let n = vertices.len();

        if normals.len() != n {
            panic!("invalid number of normals");
        }

        for face in &faces {
            for &i in face {
                if i as usize >= n {
                    panic!("index {} is out of bounds", i);
                }
            }
        }

        let mut data = vec![];
        data.extend(vertices);
        data.extend(normals);
        let data: Arc<[_]> = data.into();

        let tris = faces
            .into_iter()
            .map(move |[a, b, c]| MeshTriangle {
                vertices: [a, b, c],
                data: data.clone(),
            })
            .collect::<Vec<_>>();

        Mesh {
            tree: AABBTree::new(tris, 0.1),
        }
    }

    pub fn from_vertices(vertices: Vec<Vec3D>, faces: Vec<[u32; 3]>) -> Self {
        let n = vertices.len();
        let mut normals = vec![Vec3D::zero(); n];

        for &[i, j, k] in &faces {
            let [a, b, c] = [
                vertices[i as usize],
                vertices[j as usize],
                vertices[k as usize],
            ];

            let e1 = b - a;
            let e2 = c - a;
            let normal = Vec3D::cross(e1, e2);

            unsafe {
                *normals.get_unchecked_mut(i as usize) += normal;
                *normals.get_unchecked_mut(j as usize) += normal;
                *normals.get_unchecked_mut(k as usize) += normal;
            }
        }

        for normal in &mut normals {
            *normal = normal.normalize();
        }

        Self::new(vertices, normals, faces)
    }

    pub fn from_triangles(tris: Vec<Triangle>) -> Self {
        let mut vertices = vec![];
        let mut faces = vec![];
        let mut mapping = HashMap::new();

        for tri in tris {
            let mut face = [0, 0, 0];

            unroll! {
                for i in 0..3 {
                    let v = [tri.a, tri.b, tri.c][i];
                    let key = unsafe {
                        transmute::<Vec3D, [u8; 12]>(v)
                    };

                    use std::collections::hash_map::Entry::*;
                    let index = match mapping.entry(key) {
                        Occupied(e) => *e.get(),
                        Vacant(e) => {
                            let n = vertices.len() as u32;
                            vertices.push(v);
                            *e.insert(n)
                        },
                    };

                    face[i] = index;
                }
            }

            faces.push(face);
        }

        Self::from_vertices(vertices, faces)
    }
}

impl Geometry for MeshTriangle {
    #[inline(always)]
    fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult> {
        let data = &*self.data;
        let [i, j, k] = self.vertices;
        let a = unsafe { *data.get_unchecked(i as usize) };
        let b = unsafe { *data.get_unchecked(j as usize) };
        let c = unsafe { *data.get_unchecked(k as usize) };

        let [t, u, v] = moller_trumbore([a, b, c], ray);

        if t >= 0.0 && t <= t_max && u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            let offset = data.len() / 2;
            let na = unsafe { *data.get_unchecked(offset + i as usize) };
            let nb = unsafe { *data.get_unchecked(offset + j as usize) };
            let nc = unsafe { *data.get_unchecked(offset + k as usize) };
            let norm = (1.0 - u - v) * na + u * nb + v * nc;

            Some(HitResult {
                t,
                norm,
                pos: ray.at(t),
                material: &DEFAULT_MATERIAL,
                uv: [u, v],
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        let data = &*self.data;
        let [i, j, k] = self.vertices;
        let a = unsafe { *data.get_unchecked(i as usize) };
        let b = unsafe { *data.get_unchecked(j as usize) };
        let c = unsafe { *data.get_unchecked(k as usize) };

        AABB::from_point(a).union_point(b).union_point(c)
    }
}

impl Geometry for Mesh {
    delegate! {
        target self.tree {
            fn hit(&self, ray: &Ray, t_max: f32) -> Option<HitResult>;
            fn is_hit(&self, ray: &Ray, t_max: f32) -> bool;
            fn bounding_box(&self) -> AABB;
        }
    }
}
