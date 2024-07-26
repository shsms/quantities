use super::{Percentage, Power, Quantity, Voltage};

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

qty_ctor! {
    Current => {
        (from_milliamperes, as_milliamperes, "mA", 0.001),
        (from_amperes, as_amperes, "A", 1.0),
    }
}

impl std::ops::Mul<Voltage> for Current {
    type Output = Power;

    fn mul(self, voltage: Voltage) -> Self::Output {
        Power::from_watts(self.as_amperes() * voltage.as_volts())
    }
}
