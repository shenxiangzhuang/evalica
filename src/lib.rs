use numpy::{IntoPyArray, PyArray1, PyArray2};
use numpy::PyArrayMethods;
use pyo3::prelude::*;

mod bradley_terry;
mod newman;

#[pyfunction]
fn py_bradley_terry(py: Python, m: &Bound<PyArray2<i64>>) -> PyResult<(Py<PyArray1<f64>>, usize)> {
    let m = unsafe { m.as_array().to_owned() };
    let (pi, iterations) = bradley_terry::bradley_terry(&m);
    Ok((pi.into_pyarray_bound(py).unbind(), iterations))
}

#[pyfunction]
fn py_newman(
    py: Python,
    m: &Bound<PyArray2<i64>>,
    seed: u64,
    tolerance: f64,
    limit: usize,
) -> PyResult<(Py<PyArray1<f64>>, usize)> {
    let m = unsafe { m.as_array().to_owned() };
    let (pi, iterations) = newman::newman(&m, seed, tolerance, limit);
    Ok((pi.into_pyarray_bound(py).unbind(), iterations))
}

#[pymodule]
fn evalica(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_bradley_terry, m)?)?;
    m.add_function(wrap_pyfunction!(py_newman, m)?)?;
    Ok(())
}