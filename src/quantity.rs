mod current;
mod energy;
mod percentage;
mod power;
mod voltage;

pub use current::Current;
pub use energy::Energy;
pub use percentage::Percentage;
pub use power::Power;
pub use voltage::Voltage;

pub trait Quantity {
    fn base_value(&self) -> f64;
    fn base_unit(&self) -> &str;
}

impl std::fmt::Display for dyn Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.base_value(), self.base_unit())
    }
}
