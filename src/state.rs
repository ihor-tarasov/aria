use core::fmt;

use crate::value::Value;

pub enum VMError {
    StackOverflow,
    StackUnderflow,
    UnknownInstruction,
    OpcodeFetch,
    BinaryOperator,
    DividingByZero,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::StackOverflow => write!(f, "Stack overflow."),
            VMError::StackUnderflow => write!(f, "Stack underflow."),
            VMError::UnknownInstruction => write!(f, "Unknown instruction."),
            VMError::OpcodeFetch => write!(f, "Unable to fetch opcode."),
            VMError::BinaryOperator => write!(f, "Binary operator error."),
            VMError::DividingByZero => write!(f, "Dividing by zero."),
        }
    }
}

pub type VMResult<T> = Result<T, VMError>;

pub trait Stack {
    fn push(&mut self, value: Value) -> VMResult<()>;
    fn pop(&mut self) -> VMResult<Value>;
}

pub struct State<S> {
    stack: S,
    pub program_counter: usize,
    pub message: Option<Box<str>>,
}

impl<S: Stack> State<S> {
    pub fn new(stack: S) -> Self {
        Self {
            stack,
            program_counter: 0,
            message: None,
        }
    }

    pub fn push(&mut self, value: Value) -> VMResult<()> {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> VMResult<Value> {
        self.stack.pop()
    }

    fn binary<F>(&mut self, f: F) -> VMResult<()>
    where
        F: Fn(&mut Self, Value, Value) -> VMResult<Value>,
    {
        let right = self.pop()?;
        let left = self.pop()?;
        let result = f(self, left, right)?;
        self.push(result)
    }

    fn error<T>(&mut self, m: String, e: VMError) -> VMResult<T> {
        self.message = Some(m.into_boxed_str());
        Err(e)
    }

    pub fn single<F>(&mut self, f: F) -> VMResult<bool>
    where
        F: Fn(&mut Self) -> VMResult<()>,
    {
        f(self)?;
        self.program_counter += 1;
        Ok(true)
    }

    fn op_error(&mut self, operator: &str, l: Value, r: Value) -> VMResult<Value> {
        self.error(
            format!("Unable to use '{operator}' for {l} and {r} values."),
            VMError::BinaryOperator,
        )
    }

    fn op_addict(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l.wrapping_add(r))),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 + r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l + r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l + r)),
            _ => self.op_error("+", l, r),
        }
    }

    fn op_multiply(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l.wrapping_mul(r))),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 * r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l * r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l * r)),
            _ => self.op_error("*", l, r),
        }
    }

    fn op_subtract(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(l.wrapping_sub(r))),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 - r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l - r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l - r)),
            _ => self.op_error("-", l, r),
        }
    }

    fn op_divide(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    Err(VMError::DividingByZero)
                } else {
                    Ok(Value::Integer(l.wrapping_div(r)))
                }
            }
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 / r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l / r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l / r)),
            _ => self.op_error("/", l, r),
        }
    }

    fn op_module(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => {
                if r == 0 {
                    Err(VMError::DividingByZero)
                } else {
                    Ok(Value::Integer(l.wrapping_rem(r)))
                }
            }
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Real(l as f64 % r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Real(l % r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Real(l % r)),
            _ => self.op_error("%", l, r),
        }
    }

    fn op_less(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l < r)),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Boolean((l as f64) < r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Boolean(l < r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Boolean(l < r)),
            _ => self.op_error("<", l, r),
        }
    }

    fn op_greater(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l > r)),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Boolean((l as f64) > r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Boolean(l > r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Boolean(l > r)),
            _ => self.op_error(">", l, r),
        }
    }

    fn op_less_equals(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l <= r)),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Boolean((l as f64) <= r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Boolean(l <= r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Boolean(l <= r)),
            _ => self.op_error("<=", l, r),
        }
    }

    fn op_greater_equals(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l >= r)),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Boolean((l as f64) >= r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Boolean(l >= r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Boolean(l >= r)),
            _ => self.op_error(">=", l, r),
        }
    }

    fn op_equals(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l == r)),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Boolean((l as f64) == r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Boolean(l == r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Boolean(l == r)),
            _ => self.op_error("==", l, r),
        }
    }

    fn op_not_equals(&mut self, l: Value, r: Value) -> VMResult<Value> {
        match (l, r) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l != r)),
            (Value::Integer(l), Value::Real(r)) => Ok(Value::Boolean((l as f64) != r)),
            (Value::Real(l), Value::Integer(r)) => Ok(Value::Boolean(l != r as f64)),
            (Value::Real(l), Value::Real(r)) => Ok(Value::Boolean(l != r)),
            _ => self.op_error("!=", l, r),
        }
    }

    pub fn addict(&mut self) -> VMResult<()> {
        self.binary(Self::op_addict)
    }

    pub fn multiply(&mut self) -> VMResult<()> {
        self.binary(Self::op_multiply)
    }

    pub fn subtract(&mut self) -> VMResult<()> {
        self.binary(Self::op_subtract)
    }

    pub fn divide(&mut self) -> VMResult<()> {
        self.binary(Self::op_divide)
    }

    pub fn module(&mut self) -> VMResult<()> {
        self.binary(Self::op_module)
    }

    pub fn less(&mut self) -> VMResult<()> {
        self.binary(Self::op_less)
    }

    pub fn greater(&mut self) -> VMResult<()> {
        self.binary(Self::op_greater)
    }

    pub fn less_equals(&mut self) -> VMResult<()> {
        self.binary(Self::op_less_equals)
    }

    pub fn greater_equals(&mut self) -> VMResult<()> {
        self.binary(Self::op_greater_equals)
    }

    pub fn equals(&mut self) -> VMResult<()> {
        self.binary(Self::op_equals)
    }

    pub fn not_equals(&mut self) -> VMResult<()> {
        self.binary(Self::op_not_equals)
    }
}
