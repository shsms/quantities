mod python;
mod quantity;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "frequenz_sdk")]
fn frequenz_sdk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::quantity::register_submodule(m)
}
