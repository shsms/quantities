use super::Quantity;

#[derive(Copy, Clone)]
pub struct Percentage {
    value: f64,
}

impl Quantity for Percentage {
    fn base_value(&self) -> f64 {
        self.value
    }

    fn base_unit(&self) -> &str {
        "%"
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} %", self.value)
    }
}

/// Constructors
impl Percentage {
    pub fn from_percentage(percentage: f64) -> Self {
        Self { value: percentage }
    }

    pub fn from_fraction(fraction: f64) -> Self {
        Self {
            value: fraction * 100.0,
        }
    }
}

/// Getters
impl Percentage {
    pub fn as_percentage(&self) -> f64 {
        self.value
    }

    pub fn as_fraction(&self) -> f64 {
        self.value / 100.0
    }
}

/// Arithmetics
impl std::ops::Add for Percentage {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
        }
    }
}

impl std::ops::Sub for Percentage {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value - other.value,
        }
    }
}

impl std::ops::Mul for Percentage {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: self.as_percentage() * other.as_fraction(),
        }
    }
}

impl std::ops::Mul<f64> for Percentage {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            value: self.as_percentage() * other,
        }
    }
}
