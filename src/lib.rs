use pyo3::prelude::*;

#[pyfunction]
fn tell() -> String {
    let phrase: String = String::from("testing hello");
    phrase
}

#[pymodule]
fn hello(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tell, m)?)?;

    Ok(())
}
