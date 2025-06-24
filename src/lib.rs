use pyo3::prelude::*;
mod say;

#[pyfunction]
fn tell() -> String {
	say::tell()
	
}

#[pymodule]
fn hello(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tell, m)?)?;

    Ok(())
}
