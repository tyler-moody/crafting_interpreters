
use std::fmt;
use std::ops::Neg;
use try_traits::ops::TryAdd;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    FloatingPoint(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::FloatingPoint(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::FloatingPoint(f) => {
                Value::FloatingPoint(-f)
            }
        }
    }
}
