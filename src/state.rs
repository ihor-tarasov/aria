use core::fmt;

pub enum VMError {
    StackOverflow,
    StackUnderflow,
    UnknownInstruction,
    OpcodeFetch,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::StackOverflow => write!(f, "Stack overflow"),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::UnknownInstruction => write!(f, "Unknown instrruction"),
            VMError::OpcodeFetch => write!(f, "Unable to fetch opcode"),
        }
    }
}

pub type VMResult<T> = Result<T, VMError>;

pub trait Stack {
    fn push(&mut self, value: i64) -> VMResult<()>;
    fn pop(&mut self) -> VMResult<i64>;
}

pub struct State<S> {
    stack: S,
    pub program_counter: usize,
}

impl<S: Stack> State<S> {
    pub fn new(stack: S) -> Self {
        Self {
            stack,
            program_counter: 0,
        }
    }

    pub fn push(&mut self, value: i64) -> VMResult<()> {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> VMResult<i64> {
        self.stack.pop()
    }

    fn binary<F>(&mut self, f: F) -> VMResult<()>
    where
        F: Fn(i64, i64) -> VMResult<i64>,
    {
        let right = self.pop()?;
        let left = self.pop()?;
        self.push(f(left, right)?)
    }

    pub fn addict(&mut self) -> VMResult<()> {
        self.binary(|l, r| Ok(l.wrapping_add(r)))
    }

    pub fn multiply(&mut self) -> VMResult<()> {
        self.binary(|l, r| Ok(l.wrapping_mul(r)))
    }
}
