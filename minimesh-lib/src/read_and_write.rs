use std::fs::File;
use std::io::{BufReader, Write};
use std::fs;
use crate::{Mesh, Triangle, Vertex};
use crate::linalg::array_to_matrix;
use byteorder::{LittleEndian, WriteBytesExt};
use nalgebra::Point3;

// ===========================================================
// ================= Reading STL Files =================
// ===========================================================

// quick fix for now, implement properly later
pub fn load_stl(path: &str) -> Vec<Triangle>{
    let file = File::open(path).unwrap();
    let mut root_vase = BufReader::new(&file);
    let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();
    let mut verts;
    let mut triangles = Vec::new();
    for tri in mesh.triangles() {

        verts = [
            array_to_matrix(&tri.vertices()[0]),
            array_to_matrix(&tri.vertices()[1]),
            array_to_matrix(&tri.vertices()[2])
        ];
        triangles.push(Triangle::new(verts));
    }

    let _vertices : Vec<Vertex> = mesh
                                    .vertices()
                                    .map(|v| Point3::new(v[0] as f64, v[1] as f64, v[2] as f64))
                                    .collect();

    triangles
}


//writing mesh to STL:
pub fn export_stl(path: &str, mesh: Mesh) {
    //based on: https://en.wikipedia.org/wiki/STL_(file_format)

    let mut writer = vec![];
    let mut normal;
    //Writing STL header UINT8[80] – Header - 80 bytes
    let header = [0u8; 80];
    writer.write_all(&header).expect("Error");

    //Writing tri count
    let tri_count = mesh.tris.len();
    writer
        .write_u32::<LittleEndian>(tri_count as u32)
        .expect("Error");

    //FOR EACH TRIANGLE
    let mut tri = 0;
    // let tri_vert_count = mesh.triangles.len();
    let tri_count = mesh.tris.len();
    while tri < tri_count {
        //write triangle normal
        normal = mesh.tri_normal(tri); //calculate normal
        writer
            .write_f32::<LittleEndian>((normal.x) as f32)
            .expect("Error"); // write normal values
        writer
            .write_f32::<LittleEndian>((normal.y) as f32)
            .expect("Error");
        writer
            .write_f32::<LittleEndian>((normal.z) as f32)
            .expect("Error");
        
        //write each vertex
        let vertices = mesh.tri_coords(tri);
        for vertex in vertices {
            // write vertex coordinates
            writer
                .write_f32::<LittleEndian>(vertex[0] as f32)
                .expect("Error");
            writer
                .write_f32::<LittleEndian>(vertex[1] as f32)
                .expect("Error");
            writer
                .write_f32::<LittleEndian>(vertex[2] as f32)
                .expect("Error");
        }
        //write attribute byte count
        writer.write_u16::<LittleEndian>(0).expect("Error");
        tri += 1;

    }
    //write final stl
    fs::write(path, writer).expect("Something went wrong.");

    println!("\nVertices: {:?}", mesh.vertices.len());
    println!("Triangles: {:?}\n", tri_count);

}