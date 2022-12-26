
use std::fmt;

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

