use super::{Percentage, Power};

#[derive(Copy, Clone)]
pub struct Energy {
    value: f64,
}

qty_ctor! {
    Energy => {
        (from_milliwatthours, as_milliwatthours, "mWh", 0.001),
        (from_watthours, as_watthours, "Wh", 1.0),
        (from_kilowatthours, as_kilowatthours, "kWh", 1000.0),
        (from_megawatthours, as_megawatthours, "MWh", 1_000_000.0),
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
