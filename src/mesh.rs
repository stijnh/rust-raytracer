/*

struct MeshData {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
}

struct MeshTriangle {
    data: Rc<MeshData>,
    vertices: [u32; 3],
    normals: [u32; 3],
}

impl MeshTriangles {
    fn get_vertices(&self) -> (Vec3, Vec3, Vec3) {
        unsafe {
            let v = &self.data.vertices;
            let a = v.get_unchecked(self.vertices[0]);
            let b = v.get_unchecked(self.vertices[0]);
            let c = v.get_unchecked(self.vertices[0]);
            (a, b, c)
        }
    }
}

impl Object for MeshTriangle {
    fn hit(&self, t_min: f64, t_max: f64) -> Option<HitResult> {
        let (a, b, c) = self.get_vertices();


    }
}

struct Mesh {
    data: Rc<MeshData>,
    triangles: Vec<MeshTriangle>,
}

*/
