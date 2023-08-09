use super::data_stack::Data;

struct StaticData<const SIZE: usize>([i64; SIZE]);

impl<const SIZE: usize> StaticData<SIZE> {
    fn new() -> Self {
        Self([0; SIZE])
    }
}

impl<const SIZE: usize> Data for StaticData<SIZE> {
    fn get(&self, index: usize) -> Option<&i64> {
        self.0.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut i64> {
        self.0.get_mut(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn new<const SIZE: usize>() -> impl Data {
    StaticData::<SIZE>::new()
}
