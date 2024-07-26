use super::{Current, Percentage, Power, Quantity};

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

qty_ctor! {
    Voltage => {
        (from_millivolts, as_millivolts, 0.001),
        (from_volts, as_volts, 1.0),
        (from_kilovolts, as_kilovolts, 1000.0)
    }
}

impl std::ops::Mul<Current> for Voltage {
    type Output = Power;

    fn mul(self, current: Current) -> Self::Output {
        Power::from_watts(self.as_volts() * current.as_amperes())
    }
}
