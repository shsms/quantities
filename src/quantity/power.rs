use super::{Current, Energy, Percentage, Voltage};

#[derive(Copy, Clone)]
pub struct Power {
    value: f64,
}

qty_ctor! {
    Power => {
        (from_milliwatts, as_milliwatts, "mW", 0.001),
        (from_watts, as_watts, "W", 1.0),
        (from_kilowatts, as_kilowatts, "kW", 1000.0),
        (from_megawatts, as_megawatts, "MW", 1_000_000.0),
        (from_gigawatts, as_gigawatts, "GW", 1_000_000_000.0),
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
