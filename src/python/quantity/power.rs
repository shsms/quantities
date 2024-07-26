use crate::quantity;
use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone)]
pub(crate) struct Power(pub quantity::Power);

#[pymethods]
impl Power {
    #[staticmethod]
    fn from_watts(value: f64) -> Self {
        Self(quantity::Power::from_watts(value))
    }

    #[staticmethod]
    fn from_milliwatts(value: f64) -> Self {
        Self(quantity::Power::from_milliwatts(value))
    }

    #[staticmethod]
    fn from_kilowatts(value: f64) -> Self {
        Self(quantity::Power::from_kilowatts(value))
    }

    #[staticmethod]
    fn from_megawatts(value: f64) -> Self {
        Self(quantity::Power::from_megawatts(value))
    }

    fn as_watts(&self) -> f64 {
        self.0.as_watts()
    }

    fn as_milliwatts(&self) -> f64 {
        self.0.as_milliwatts()
    }

    fn as_kilowatts(&self) -> f64 {
        self.0.as_kilowatts()
    }

    fn as_megawatts(&self) -> f64 {
        self.0.as_megawatts()
    }

    fn __repr__(&self) -> String {
        format!("Power.from_watts({})", self.0.as_watts())
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __add__(&self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn __sub__(&self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
