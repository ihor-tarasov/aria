use crate::get::GetByte;

pub(super) struct BoxedGet(Box<[u8]>);

impl BoxedGet {
    pub(super) fn new(slice: Box<[u8]>) -> Self {
        Self(slice)
    }
}

impl GetByte for BoxedGet {
    fn get_byte(&self, address: usize) -> Option<u8> {
        self.0.get(address).cloned()
    }
}
