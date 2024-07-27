use super::{Current, Energy, Percentage, Voltage};

#[derive(Copy, Clone)]
pub struct Power {
    value: f64,
}

qty_ctor! {
    Power => {
        (from_milliwatts, as_milliwatts, "mW", 10e-3),
        (from_watts, as_watts, "W", 10e0),
        (from_kilowatts, as_kilowatts, "kW", 10e3),
        (from_megawatts, as_megawatts, "MW", 10e6),
        (from_gigawatts, as_gigawatts, "GW", 10e9),
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
