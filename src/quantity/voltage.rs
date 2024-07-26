use super::{Current, Percentage, Power};

#[derive(Copy, Clone)]
pub struct Voltage {
    value: f64,
}

qty_ctor! {
    Voltage => {
        (from_millivolts, as_millivolts, "mV", 0.001),
        (from_volts, as_volts, "V", 1.0),
        (from_kilovolts, as_kilovolts, "kV", 1000.0),
    }
}

impl std::ops::Mul<Current> for Voltage {
    type Output = Power;

    fn mul(self, current: Current) -> Self::Output {
        Power::from_watts(self.as_volts() * current.as_amperes())
    }
}
