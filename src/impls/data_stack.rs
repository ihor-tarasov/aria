use crate::{
    state::Stack,
    state::{VMError, VMResult},
    value::Value,
};

pub trait Data {
    fn get(&self, index: usize) -> Option<&Value>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Value>;
    fn len(&self) -> usize;
}

struct DataStack<D> {
    data: D,
    top: usize,
}

impl<D> DataStack<D> {
    fn new(data: D) -> Self {
        Self { data, top: 0 }
    }
}

impl<D: Data> Stack for DataStack<D> {
    fn push(&mut self, value: Value) -> VMResult<()> {
        self.data
            .get_mut(self.top)
            .and_then(|d| {
                self.top += 1;
                Some(*d = value)
            })
            .ok_or(VMError::StackOverflow)
    }

    fn pop(&mut self) -> VMResult<Value> {
        if self.top == 0 {
            Err(VMError::StackUnderflow)
        } else {
            self.top -= 1;
            self.data
                .get(self.top)
                .cloned()
                .ok_or(VMError::StackOverflow)
        }
    }
}

pub fn new<D: Data>(data: D) -> impl Stack {
    DataStack::new(data)
}
