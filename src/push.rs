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

impl<P: PushByte> PushData<i64> for P {
    fn push_data(&mut self, value: i64) {
        for b in value.to_be_bytes().iter().cloned() {
            self.push_byte(b);
        }
    }
}
