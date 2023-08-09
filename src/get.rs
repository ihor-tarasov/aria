pub trait GetByte {
    fn get_byte(&self, address: usize) -> Option<u8>;
}

pub trait GetData<T> {
    fn get_data(&self, address: usize) -> Option<T>;
}

macro_rules! impl_get_data {
    ($($t:ty),*) => {
        $(
            impl<G: GetByte> GetData<$t> for G {
                fn get_data(&self, address: usize) -> Option<$t> {
                    let mut is_completed = true;
                    let r = core::array::from_fn(|i| match self.get_byte(address + i) {
                        Some(b) => b,
                        None => {
                            is_completed = false;
                            0
                        }
                    });
                    if is_completed {
                        Some(<$t>::from_be_bytes(r))
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

impl_get_data!(i64, f64);
