pub trait GetByte {
    fn get_byte(&self, address: usize) -> Option<u8>;
}

pub trait GetData<T> {
    fn get_data(&self, address: usize) -> Option<T>;
}

impl<G: GetByte> GetData<i64> for G {
    fn get_data(&self, address: usize) -> Option<i64> {
        let mut is_completed = true;
        let r = core::array::from_fn(|i| match self.get_byte(address + i) {
            Some(b) => b,
            None => {
                is_completed = false;
                0
            }
        });
        if is_completed {
            Some(i64::from_be_bytes(r))
        } else {
            None
        }
    }
}
