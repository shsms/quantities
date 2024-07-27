use super::{Percentage, Power, Voltage};

#[derive(Copy, Clone)]
pub struct Current {
    value: f64,
}

qty_ctor! {
    Current => {
        (from_milliamperes, as_milliamperes, "mA", 10e-3),
        (from_amperes, as_amperes, "A", 10e0),
    }
}

impl std::ops::Mul<Voltage> for Current {
    type Output = Power;

    fn mul(self, voltage: Voltage) -> Self::Output {
        Power::from_watts(self.as_amperes() * voltage.as_volts())
    }
}
