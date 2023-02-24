use minimesh::{Mesh, calculate_surface_area, calculate_volume};

fn main() {

    let stl_mesh = Mesh::from_stl("../stl/cube.stl");

    println!("AABB: {:?}", stl_mesh.aabb());
    println!("Dimensions: {:?}", stl_mesh.dimensions());
    println!("Volume: {}", calculate_volume(&stl_mesh));
    println!("surface_area: {}", calculate_surface_area(&stl_mesh));

}
