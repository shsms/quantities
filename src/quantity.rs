pub trait Quantity {
    fn base_value(&self) -> f64;
    fn base_unit(&self) -> &str;
}

impl std::fmt::Display for dyn Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.base_value(), self.base_unit())
    }
}

macro_rules! qty_ctor {
    (@impl ($ctor_method:ident, $get_method:ident, $multiple:literal)) => {
        pub fn $ctor_method(value: f64) -> Self {
            Self { value: value * $multiple }
        }
        pub fn $get_method(&self) -> f64 {
            self.value / $multiple
        }
    };
    (@impl ($ctor_method:ident, $get_method:ident, $multiple:literal), $($rest:tt)*) => {
        qty_ctor!(@impl ($ctor_method, $get_method, $multiple));
        qty_ctor!(@impl $($rest)*);
    };
    (@impl_arith_ops $typename:ident) => {
        impl std::ops::Add for $typename {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value + rhs.value,
                }
            }
        }

        impl std::ops::Sub for $typename {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value - rhs.value,
                }
            }
        }

        impl std::ops::Mul<Percentage> for $typename {
            type Output = Self;

            fn mul(self, other: Percentage) -> Self::Output {
                Self {
                    value: self.value * other.as_fraction(),
                }
            }
        }

        impl std::ops::Mul<f64> for $typename {
            type Output = Self;

            fn mul(self, other: f64) -> Self::Output {
                Self {
                    value: self.value * other,
                }
            }
        }
    };
    ($typename:ident => {$($rest:tt)*}) => {
        impl $typename {
            qty_ctor!(@impl $($rest)*);
        }

        qty_ctor!{@impl_arith_ops $typename}
    };
}

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
