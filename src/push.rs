use crate::get::GetByte;

pub trait PushByte {
    fn push_byte(&mut self, value: u8);
}

pub trait IntoGetByte {
    type Target: GetByte;
    fn into_get_byte(self) -> Self::Target;
}

pub trait PushData<T> {
    fn push_data(&mut self, value: T);
}

macro_rules! impl_push_data {
    ($($t:ty),*) => {
        $(
            impl<P: PushByte> PushData<$t> for P {
                fn push_data(&mut self, value: $t) {
                    for b in value.to_be_bytes().iter().cloned() {
                        self.push_byte(b);
                    }
                }
            }
        )*
    };
}

impl_push_data!(i64, f64);
