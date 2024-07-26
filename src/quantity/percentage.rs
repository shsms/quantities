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

qty_ctor! {
    Percentage => {
        (from_percentage, as_percentage, "%", 1.0),
        (from_fraction, as_fraction, None, 100.0),
    }
}
