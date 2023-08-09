use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Void,
    Boolean(bool),
    Integer(i64),
    Real(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Void => write!(f, "()"),
            Value::Boolean(value) => write!(f, "{value}"),
            Value::Integer(value) => write!(f, "{value}"),
            Value::Real(value) => write!(f, "{value}"),
        }
    }
}
