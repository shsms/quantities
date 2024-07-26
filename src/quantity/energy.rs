use crate::quantity::{Power, Quantity};

#[derive(Copy, Clone)]
pub struct Energy {
    value: f64,
}

impl Quantity for Energy {
    fn base_value(&self) -> f64 {
        self.value
    }

    fn base_unit(&self) -> &str {
        "Wh"
    }
}

// impl std::fmt::Display for Energy {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{} Wh", self.value)
//     }
// }

/// Constructors
impl Energy {
    pub fn from_watthours(watthours: f64) -> Energy {
        Energy { value: watthours }
    }

    pub fn from_milliwatthours(milliwatthours: f64) -> Energy {
        Energy {
            value: milliwatthours / 1000.0,
        }
    }

    pub fn from_kilowatthours(kilowatthours: f64) -> Energy {
        Energy {
            value: kilowatthours * 1000.0,
        }
    }

    pub fn from_megawatthours(megawatthours: f64) -> Energy {
        Energy {
            value: megawatthours * 1_000_000.0,
        }
    }
}

/// Getters
impl Energy {
    pub fn as_watthours(&self) -> f64 {
        self.value
    }

    pub fn as_milliwatthours(&self) -> f64 {
        self.value * 1000.0
    }

    pub fn as_kilowatthours(&self) -> f64 {
        self.value / 1000.0
    }

    pub fn as_megawatthours(&self) -> f64 {
        self.value / 1_000_000.0
    }
}

/// Arithmetics
impl std::ops::Add for Energy {
    type Output = Self;

    fn add(self, rhs: Energy) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::ops::Sub for Energy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl std::ops::Div<Power> for Energy {
    type Output = std::time::Duration;

    fn div(self, power: Power) -> Self::Output {
        let seconds = (self.as_watthours() / power.as_watts()) * 3600.0;
        std::time::Duration::from_secs_f64(seconds)
    }
}

impl std::ops::Div<std::time::Duration> for Energy {
    type Output = Power;

    fn div(self, duration: std::time::Duration) -> Self::Output {
        Power::from_watts(self.as_watthours() / duration.as_secs_f64() / 3600.0)
    }
}
