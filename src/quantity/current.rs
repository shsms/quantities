use crate::quantity::{Power, Quantity, Voltage};

use super::Percentage;

#[derive(Copy, Clone)]
pub struct Current {
    value: f64,
}

impl Quantity for Current {
    fn base_value(&self) -> f64 {
        self.value
    }

    fn base_unit(&self) -> &str {
        "A"
    }
}

// impl std::fmt::Display for Current {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{} A", self.value)
//     }
// }

/// Constructors
impl Current {
    pub fn from_amperes(amps: f64) -> Self {
        Self { value: amps }
    }

    pub fn from_milliamperes(milliamps: f64) -> Self {
        Self {
            value: milliamps / 1000.0,
        }
    }
}

/// Getters
impl Current {
    pub fn as_amperes(&self) -> f64 {
        self.value
    }

    pub fn as_milliamperes(&self) -> f64 {
        self.value * 1000.0
    }
}

/// Arithmetics
impl std::ops::Add for Current {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
        }
    }
}

impl std::ops::Sub for Current {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value - other.value,
        }
    }
}

impl std::ops::Mul<Voltage> for Current {
    type Output = Power;

    fn mul(self, voltage: Voltage) -> Self::Output {
        Power::from_watts(self.as_amperes() * voltage.as_volts())
    }
}

impl std::ops::Mul<f64> for Current {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_amperes(self.as_amperes() * rhs)
    }
}

impl std::ops::Mul<Percentage> for Current {
    type Output = Self;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Self::from_amperes(self.as_amperes() * rhs.as_fraction())
    }
}
