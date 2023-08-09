use crate::value::Value;

#[derive(Debug)]
pub enum VMError {
    StackOverflow,
    StackUnderflow,
    UnknownInstruction,
    OpcodeFetch,
    BinaryOperator,
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

    fn op_error(&mut self, operator: &str, l: Value, r: Value) -> VMResult<Value> {
        self.message =
            Some(format!("Unable to use '{operator}' for {l} and {r} values.").into_boxed_str());
        Err(VMError::BinaryOperator)
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

    pub fn addict(&mut self) -> VMResult<()> {
        self.binary(Self::op_addict)
    }

    pub fn multiply(&mut self) -> VMResult<()> {
        self.binary(Self::op_multiply)
    }
}
