use crate::push::{IntoGetByte, PushByte};

use super::boxed_get::BoxedGet;

struct VecPush(Vec<u8>);

impl VecPush {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl PushByte for VecPush {
    fn push_byte(&mut self, value: u8) {
        self.0.push(value)
    }
}

impl IntoGetByte for VecPush {
    type Target = BoxedGet;

    fn into_get_byte(self) -> Self::Target {
        BoxedGet::new(self.0.into_boxed_slice())
    }
}

pub fn new() -> impl PushByte + IntoGetByte {
    VecPush::new()
}
