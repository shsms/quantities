use super::{util, Power};
use crate::quantity;

use pyo3::{prelude::*, types::PyDelta};

#[pyclass]
#[derive(Copy, Clone)]
pub(crate) struct Energy(pub quantity::Energy);

#[pymethods]
impl Energy {
    #[staticmethod]
    fn from_watthours(value: f64) -> Self {
        Self(quantity::Energy::from_watthours(value))
    }

    #[staticmethod]
    fn from_milliwatthours(value: f64) -> Self {
        Self(quantity::Energy::from_milliwatthours(value))
    }

    #[staticmethod]
    fn from_kilowatthours(value: f64) -> Self {
        Self(quantity::Energy::from_kilowatthours(value))
    }

    #[staticmethod]
    fn from_megawatthours(value: f64) -> Self {
        Self(quantity::Energy::from_megawatthours(value))
    }

    fn as_watthours(&self) -> f64 {
        self.0.as_watthours()
    }

    fn as_milliwatthours(&self) -> f64 {
        self.0.as_milliwatthours()
    }

    fn as_kilowatthours(&self) -> f64 {
        self.0.as_kilowatthours()
    }

    fn as_megawatthours(&self) -> f64 {
        self.0.as_megawatthours()
    }

    fn __repr__(&self) -> String {
        format!("Energy.from_watthours({})", self.0.as_watthours())
    }

    fn __str__(&self) -> String {
        (&self.0 as &dyn quantity::Quantity).to_string()
    }

    fn __add__(&self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn __sub__(&self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }

    fn __truediv__<'py>(&self, py: Python<'py>, rhs: Power) -> PyResult<Bound<'py, PyDelta>> {
        util::duration_to_timedelta(py, self.0 / rhs.0)
    }
}
