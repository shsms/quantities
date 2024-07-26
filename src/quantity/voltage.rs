use crate::quantity::{Current, Power, Quantity};

#[derive(Copy, Clone)]
pub struct Voltage {
    value: f64,
}

impl Quantity for Voltage {
    fn base_value(&self) -> f64 {
        self.value
    }

    fn base_unit(&self) -> &str {
        "V"
    }
}

impl std::fmt::Display for Voltage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} V", self.value)
    }
}

/// Constructors
impl Voltage {
    pub fn from_volts(volts: f64) -> Self {
        Self { value: volts }
    }

    pub fn from_millivolts(millivolts: f64) -> Self {
        Self {
            value: millivolts / 1000.0,
        }
    }

    pub fn from_kilovolts(kilovolts: f64) -> Self {
        Self {
            value: kilovolts * 1000.0,
        }
    }
}

/// Getters
impl Voltage {
    pub fn as_volts(&self) -> f64 {
        self.value
    }

    pub fn as_millivolts(&self) -> f64 {
        self.value * 1000.0
    }

    pub fn as_kilovolts(&self) -> f64 {
        self.value / 1000.0
    }
}

/// Arithmetics
impl std::ops::Add for Voltage {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Sub for Voltage {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::Mul<Current> for Voltage {
    type Output = Power;

    fn mul(self, current: Current) -> Self::Output {
        Power::from_watts(self.as_volts() * current.as_amperes())
    }
}
