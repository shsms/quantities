// Formats an f64 with a given precision and remove trailing zeros
fn format_float(value: f64, precision: usize) -> String {
    let mut s = format!("{:.1$}", value, precision);
    if s.contains('.') {
        s = s.trim_end_matches('0').to_string();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

macro_rules! qty_format {
    (@impl $self:ident, $f:ident, $prec:ident,
        ($ctor:ident, $getter:ident, $unit:literal, $exp:literal),
    ) => {
        write!($f, "{} {}", format_float( $self.$getter(), $prec), $unit)
    };

    (@impl $self:ident, $f:ident, $prec:ident,
        ($ctor1:ident, $getter1:ident, $unit1:literal, $exp1:literal),
        ($ctor2:ident, $getter2:ident, $unit2:literal, $exp2:literal), $($rest:tt)*
    ) => {{
        const {assert!($exp1 < $exp2, "Units must be in increasing order of magnitude.")};

        if $exp1 <= $self.value && $self.value < $exp2 {
            write!($f, "{} {}", format_float( $self.$getter1(), $prec), $unit1)
        } else {
            qty_format!(@impl $self, $f, $prec, ($ctor2, $getter2, $unit2, $exp2), $($rest)*)
        }}
    };

    (@impl $self:ident, $f:ident, $prec:ident,
        ($ctor1:ident, $getter1:ident, $unit1:literal, $exp1:literal),
        ($ctor2:ident, $getter2:ident, None, $exp2:literal),
    ) => {
        write!($f, "{} {}", format_float( $self.$getter1(), $prec), $unit1)
    };

    (@start $self:ident, $f:ident, $prec:ident,
        ($ctor:ident, $getter:ident, $unit:literal, $exp:literal), $($rest:tt)*
    ) => {
        if $self.value <= $exp {
            write!($f, "{} {}", format_float( $self.$getter(), $prec), $unit)
        } else {
            qty_format!(@impl $self, $f, $prec, ($ctor, $getter, $unit, $exp), $($rest)*)
        }
    };

    ($typename:ident => {$($rest:tt)*}) => {
        use super::format_float;
        impl std::fmt::Display for $typename {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let prec = if let Some(prec) = f.precision() {
                    prec
                } else {
                    3
                };
                qty_format!(@start self, f, prec, $($rest)*)
            }

        }
    };
}

macro_rules! qty_ctor {
    (@impl ($ctor:ident, $getter:ident, $unit:tt, $exp:literal) $(,)?) => {
        pub fn $ctor(value: f64) -> Self {
            Self { value: value * $exp }
        }
        pub fn $getter(&self) -> f64 {
            self.value / $exp
        }
    };
    (@impl ($ctor:ident, $getter:ident, $unit:tt, $exp:literal), $($rest:tt)*) => {
        qty_ctor!(@impl ($ctor, $getter, $unit, $exp));
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
        qty_format!{$typename => {$($rest)*}}
    };
}

mod current;
mod energy;
mod percentage;
mod power;
mod temperature;
mod voltage;

pub use current::Current;
pub use energy::Energy;
pub use percentage::Percentage;
pub use power::Power;
pub use voltage::Voltage;
