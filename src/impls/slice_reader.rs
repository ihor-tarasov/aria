use crate::lexer::Reader;

struct SliceReader<'a> {
    slice: &'a [u8],
    offset: usize,
}

impl<'a> SliceReader<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice, offset: 0 }
    }
}

impl<'a> Reader for SliceReader<'a> {
    fn current(&mut self) -> Option<u8> {
        self.slice.get(self.offset).cloned()
    }

    fn advance(&mut self) {
        self.offset += 1;
    }

    fn offset(&self) -> usize {
        self.offset
    }
}

pub fn new(slice: &[u8]) -> impl Reader + '_ {
    SliceReader::new(slice.as_ref())
}
