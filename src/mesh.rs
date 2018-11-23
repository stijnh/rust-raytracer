struct MeshTriangle {
    data: Rc<[Vec3D]>,
    vertices: [u32; 3],
    normals: [u32; 3],
}

impl MeshTriangle {
    fn vertices(&self) -> (Vec3, Vec3, Vec3) {
        unsafe {
            let v = &self.data;
            let a = v.get_unchecked(self.vertices[0]);
            let b = v.get_unchecked(self.vertices[1]);
            let c = v.get_unchecked(self.vertices[2]);
            (a, b, c)
        }
    }
}

impl Object for MeshTriangle {
    fn hit(&self, t_min: f64, t_max: f64) -> Option<HitResult> {
        let (a, b, c) = self.vertices();
        Triangle::new(a, b, c).hit(t_min, t_max)
    }

    fn bounding_box(&self) -> AABB {
        let (a, b, c) = self.vertices();
        Triangle::new(a, b, c).bounding_box()
    }
}

struct Mesh {
    triangles: Vec<MeshTriangle>,
}
