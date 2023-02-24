
use pyo3::prelude::*;
use minimesh_lib::{Mesh, calculate_surface_area, calculate_volume};


#[pyfunction]
fn mesh_volume(path : &str) -> PyResult<f64> {

    let stl_mesh = Mesh::from_stl(path);

    Ok(calculate_volume(&stl_mesh))
}

#[pyfunction]
fn mesh_area(path : &str) -> PyResult<f64> {

    let stl_mesh = Mesh::from_stl(path);

    Ok(calculate_surface_area(&stl_mesh))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn minimesh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(mesh_volume, m)?)?;
    m.add_function(wrap_pyfunction!(mesh_area, m)?)?;
    Ok(())
}