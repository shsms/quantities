use super::Percentage;

#[derive(Copy, Clone)]
pub struct Temperature {
    value: f64,
}

qty_ctor! {
    Temperature => {
        (from_celsius, as_celsius, "°C", 1.0),
    }
}
