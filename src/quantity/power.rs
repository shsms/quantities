use crate::quantity::{Current, Energy, Quantity, Voltage};

#[derive(Copy, Clone)]
pub struct Power {
    value: f64,
}

impl Quantity for Power {
    fn base_value(&self) -> f64 {
        self.value
    }

    fn base_unit(&self) -> &str {
        "W"
    }
}

// impl std::fmt::Display for Power {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{} W", self.value)
//     }
// }

/// Constructors
impl Power {
    pub fn from_watts(watts: f64) -> Power {
        Power { value: watts }
    }

    pub fn from_milliwatts(milliwatts: f64) -> Power {
        Power {
            value: milliwatts / 1000.0,
        }
    }

    pub fn from_kilowatts(kilowatts: f64) -> Power {
        Power {
            value: kilowatts * 1000.0,
        }
    }

    pub fn from_megawatts(megawatts: f64) -> Power {
        Power {
            value: megawatts * 1_000_000.0,
        }
    }
}

/// Getters
impl Power {
    pub fn as_watts(&self) -> f64 {
        self.value
    }

    pub fn as_milliwatts(&self) -> f64 {
        self.value * 1000.0
    }

    pub fn as_kilowatts(&self) -> f64 {
        self.value / 1000.0
    }

    pub fn as_megawatts(&self) -> f64 {
        self.value / 1_000_000.0
    }
}

/// Arithmetics
impl std::ops::Add for Power {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Sub for Power {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::Div<Voltage> for Power {
    type Output = Current;

    fn div(self, voltage: Voltage) -> Self::Output {
        Current::from_amperes(self.as_watts() / voltage.as_volts())
    }
}

impl std::ops::Div<Current> for Power {
    type Output = Voltage;

    fn div(self, current: Current) -> Self::Output {
        Voltage::from_volts(self.as_watts() / current.as_amperes())
    }
}

impl std::ops::Mul<std::time::Duration> for Power {
    type Output = Energy;

    fn mul(self, duration: std::time::Duration) -> Self::Output {
        Energy::from_watthours(self.as_watts() * duration.as_secs_f64() / 3600.0)
    }
}
