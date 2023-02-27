
use pyo3::prelude::*;
use minimesh_lib::{Mesh, calculate_surface_area, calculate_volume};


#[pyfunction]
fn volume(path : &str) -> PyResult<f64> {

    let stl_mesh = Mesh::from_stl(path);

    Ok(calculate_volume(&stl_mesh))
}

#[pyfunction]
fn area(path : &str) -> PyResult<f64> {

    let stl_mesh = Mesh::from_stl(path);

    Ok(calculate_surface_area(&stl_mesh))
}

#[pyfunction]
fn dimensions(path : &str) -> PyResult<[f64; 3]> {

    let stl_mesh = Mesh::from_stl(path);


    Ok(stl_mesh.dimensions())
}

#[pyfunction]
fn vertices(path : &str) -> PyResult<Vec<[f64; 3]>> {

    let stl_mesh = Mesh::from_stl(path);

    let vertices = stl_mesh.vertices.iter().map(|x| [x[0], x[1], x[2]]).collect();


    Ok(vertices)
}

#[pyfunction]
fn triangles(path : &str) -> PyResult<Vec<[usize; 3]>> {

    let stl_mesh = Mesh::from_stl(path);

    let vertices = stl_mesh.tris.iter().map(|x| [x[0], x[1], x[2]]).collect();

    Ok(vertices)
}

/// A Python module implemented in Rust.
#[pymodule]
fn minimesh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(volume, m)?)?;
    m.add_function(wrap_pyfunction!(area, m)?)?;
    m.add_function(wrap_pyfunction!(dimensions, m)?)?;
    m.add_function(wrap_pyfunction!(vertices, m)?)?;
    m.add_function(wrap_pyfunction!(triangles, m)?)?;
    Ok(())
}

// https://pyo3.rs/v0.16.3/class/object
