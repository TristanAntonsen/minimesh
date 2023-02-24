use nalgebra::{Matrix3, Vector3, Point3};
pub mod read_and_write;
pub mod linalg;

use std::fs::File;
use std::io::BufReader;


// ==========================================================
// ======================= Data types =======================
// ==========================================================
type Vertex = Point3<f64>;

// ==========================================================
// ========================= Meshes =========================
// ==========================================================

#[derive(Clone)]
pub struct Mesh {
    // vertices : [[x1, y1, z1], [x2, y2, z2]...]
    pub vertices: Vec<Vertex>,
    
    // tris : [[v0, v1, v2], [v3, v4, v5]]
    pub tris: Vec<[usize; 3]> // new triangles
}

impl Mesh {
    // create a new Mesh
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<usize>) -> Self {
        let tris = to_tris(triangles.clone());
        Self {
            vertices: vertices,
            tris: tris
        }
    }
    // returns min/max corners of AABB
    pub fn aabb(&self) -> [Vertex; 2] {
        let vertices = self.vertices.clone();
        let mut xs = vertices.iter().map(|p| p[0]).collect::<Vec<f64>>();
        let mut ys = vertices.iter().map(|p| p[1]).collect::<Vec<f64>>();
        let mut zs = vertices.iter().map(|p| p[2]).collect::<Vec<f64>>();

        float_ord::sort(&mut xs);
        let x_min = xs[0];
        let x_max = xs[xs.len() - 1];

        float_ord::sort(&mut ys);
        let y_min = ys[0];
        let y_max = ys[ys.len() - 1];

        float_ord::sort(&mut zs);
        let z_min = zs[0];
        let z_max = zs[zs.len() - 1];

        [
            Point3::new(x_min, y_min, z_min),
            Point3::new(x_max, y_max, z_max)
        ]
    }

    pub fn dimensions(&self) -> [f64; 3] {
        let aabb = self.aabb();


        [
            aabb[1].x - aabb[0].x,
            aabb[1].y - aabb[0].y,
            aabb[1].z - aabb[0].z
        ]
    }
    
    //create a new empty Mesh
    pub fn new_empty() -> Self {
        Self { vertices: Vec::new(), tris: Vec::new()}
    }

    // try to consolidate with load_stl() later
    pub fn from_stl(path : &str) -> Self {

        let file = File::open(path).unwrap();
        let mut root_vase = BufReader::new(&file);
        let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut n = 0;
        for tri in mesh.triangles() {
            let verts_as_points = tri.vertices()
                                .map(|v| Point3::new(v[0] as f64, v[1] as f64, v[2] as f64));

            vertices.push(verts_as_points[0]);
            vertices.push(verts_as_points[1]);
            vertices.push(verts_as_points[2]);

            triangles.push([n, n + 1, n + 2]);

            n += 3;
        }

        Self {
            vertices,
            tris : triangles
        }
    
    }

    //return triangle vertex coordinates
    pub fn tri_coords(&self, tri: usize) -> Vec<Vertex> {

        let va = self.vertices[self.tris[tri][0]];
        let vb = self.vertices[self.tris[tri][1]];
        let vc = self.vertices[self.tris[tri][2]];

        vec![va, vb, vc]
    }

    //return triangle normal
    pub fn tri_normal(&self, tri: usize) -> Vector3<f64> {
        //tri = starting vertex index

        let va = self.vertices[self.tris[tri][0]];
        let vb = self.vertices[self.tris[tri][1]];
        let vc = self.vertices[self.tris[tri][2]];

        let _a = Vector3::new(va[0],va[1],va[2]);
        let _b = Vector3::new(vb[0],vb[1],vb[2]);
        let _c = Vector3::new(vc[0],vc[1],vc[2]);

        let v_a_b = _b - _a;
        let v_b_c = _c - _b;

        let cross = v_a_b.cross(&v_b_c);

        cross / cross.norm() //normal vector
    }

    //create a triangle from vertex indices
    pub fn triangle_from_verts(&mut self, x: usize, y: usize, z: usize) {
        // x/y/z are indices
        // create a triangle from existing vertices (assumes already has vertices)
        let tri = [x, y, z];

        self.tris.push(tri)
    }

    //linear transformation: translation
    pub fn translate(&mut self, vector: Vec<f64>) {
        let x = vector[0];
        let y = vector[1];
        let z = vector[2];

        for vertex in 0..self.vertices.len() {
            self.vertices[vertex][0] += x;
            self.vertices[vertex][1] += y;
            self.vertices[vertex][2] += z;
        }

    }

    //linear transformation: rotation
    pub fn rotate(&mut self, rot_mat: Matrix3<f64>) {
        let mut new_vertex;

        for vertex in 0..self.vertices.len() { // need to optimize this
            let x = self.vertices[vertex][0];
            let y = self.vertices[vertex][1];
            let z = self.vertices[vertex][2];

            new_vertex = Vector3::new(x,y,z);
            new_vertex =  rot_mat * new_vertex;

            self.vertices[vertex][0] = new_vertex[0];
            self.vertices[vertex][1] = new_vertex[1];
            self.vertices[vertex][2] = new_vertex[2];
        }
        
    }
    //copies self to another point *and merges with previous self*
    pub fn copy_to_point(&mut self, point: Vec<f64>) {

        let mut copied = self.clone();
        self.translate(point);
        self.merge(&mut copied);
        
    }

    //merge separate Meshes into one single Mesh
    pub fn merge(&mut self, other: &mut Mesh) {
        let mut new_tris = Vec::new();
        let vert_count = self.vertices.len();

        let mut updated_tri;
        for tri in &other.tris {
            updated_tri = [
                tri[0] + vert_count,
                tri[1] + vert_count,
                tri[2] + vert_count
            ];
            new_tris.push(updated_tri);
        }
        self.vertices.append(&mut other.vertices);
        self.tris.append(&mut new_tris);
    }

    pub fn triangle_info(&self) -> Vec<Triangle> {
        let mut verts;
        let mut triangles = Vec::new();
        for tri in &self.tris {
            verts = tri.map(|v| 
                Vector3::new(
                    self.vertices[v][0], 
                    self.vertices[v][1], 
                    self.vertices[v][2]
                )
            );
            triangles.push(Triangle::new(verts))
        };
        triangles
    }
}
pub struct Triangle {
    pub vertices: [Vector3<f64>; 3],
    pub edge_vectors: [Vector3<f64>; 3],
    pub dimensions: [f64;3],
    pub normal: Vector3<f64>,
    pub bbox_center: Vector3<f64>,
    pub aabb: [Vector3<f64>; 2],

}

impl Triangle {

    pub fn new(points: [Vector3<f64>; 3]) -> Self {
        // edge vectors
        let f0 = points[1] - points[0];
        let f1 = points[2] - points[1];
        let f2 = points[0] - points[2];
        let edge_vectors = [f0, f1, f2];

        // normal
        let v01 = edge_vectors[0];
        let v12 = edge_vectors[1];

        let mut normal = v12.cross(&v01);

        normal = normal.normalize();

        // bounding box
        let mut xs = points.into_iter().map(|p| p[0]).collect::<Vec<f64>>();
        float_ord::sort(&mut xs);
        let x_min = xs[0];
        let x_max = xs[xs.len() - 1];
        
        let mut ys = points.into_iter().map(|p| p[1]).collect::<Vec<f64>>();
        float_ord::sort(&mut ys);
        let y_min = ys[0];
        let y_max = ys[ys.len() - 1];

        let mut zs = points.into_iter().map(|p| p[2]).collect::<Vec<f64>>();
        float_ord::sort(&mut zs);

        let z_min = zs[0];
        let z_max = zs[zs.len() - 1];

        let x_dim = x_max - x_min;
        let y_dim = y_max - y_min;
        let z_dim = z_max - z_min;

        let dimensions = [
            x_dim,
            y_dim,
            z_dim
        ];

        let bbox_center = Vector3::new(
            (x_dim) / 2.0 + x_min,
            (y_dim) / 2.0 + y_min,
            (z_dim) / 2.0 + z_min,
        );

        Self {
            vertices: points,
            edge_vectors: edge_vectors,
            dimensions: dimensions,
            normal: normal,
            bbox_center: bbox_center,
            aabb : [Vector3::new(x_min, y_min, z_min),Vector3::new(x_max, y_max, z_max)]
        }
    }

    pub fn center_on_point(&self, point: Vector3<f64>) -> Self {
        //translated points
        let points = self.vertices
                    .map(|p| p - point);

        let new_aabb = self.aabb
                    .map(|p| p - point);
        // edge vectors
        let f0 = points[1] - points[0];
        let f1 = points[2] - points[1];
        let f2 = points[0] - points[2];

        let edge_vectors = [f0, f1, f2];

        Self {
            vertices: points,
            edge_vectors: edge_vectors,
            dimensions: self.dimensions,
            normal: self.normal,
            bbox_center: point,
            aabb : new_aabb
        }
    }
}

// moving verts into list of arrays
pub fn to_tris(verts: Vec<usize>) -> Vec<[usize; 3]> {
    // verts = "triangles" (vertex indices)
    let mut tris = Vec::new();
    let mut i = 0;

    while i < verts.len() {
        tris.push([verts[i], verts[i+1], verts[i+2]]);
        i += 3;
    }

    return tris
}

// ===========================================================
// ==================== Mesh Calculations ====================
// ===========================================================

pub fn calculate_volume(mesh: &Mesh) -> f64 {
    let triangles = &mesh.triangle_info();
    let mut volume = 0.0;
        
    for triangle in triangles {

        let cross = triangle.vertices[0].cross(&triangle.vertices[1]);
        let dot = cross.dot(&triangle.vertices[2]);

        let v = (1.0 / 6.0) * dot;
        volume = volume + v;
    }
    
    volume
}

pub fn calculate_surface_area(mesh: &Mesh) -> f64 {

    let triangles = &mesh.triangle_info();
    let mut area = 0.0;
    
    for triangle in triangles {
        let tri_area = calculate_triangle_area(&triangle);
        area = area + tri_area;
    }
    
    area
}

pub fn calculate_triangle_area(triangle: &Triangle) -> f64 {

    let a = triangle.vertices[0];
    let b = triangle.vertices[1];
    let c = triangle.vertices[2];

    let ab = Vector3::new(
        b[0]-a[0],
        b[1]-a[1],
        b[2]-a[2]
    );

    let ac = Vector3::new(
        c[0]-a[0],
        c[1]-a[1],
        c[2]-a[2]
    );
    
    let cross = ab.cross(&ac);

    let area = cross.norm() / 2.0;

    area
}
