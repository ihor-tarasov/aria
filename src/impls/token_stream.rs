use crate::{compiler::Stream, lexer::Reader, token::TokenAndPos};

struct TokenIterator<R>(R);

impl<R> TokenIterator<R> {
    fn new(reader: R) -> Self {
        Self(reader)
    }
}

impl<R: Reader> Iterator for TokenIterator<R> {
    type Item = TokenAndPos;

    fn next(&mut self) -> Option<Self::Item> {
        crate::lexer::lex(&mut self.0)
    }
}

struct TokenStream<I: Iterator>(core::iter::Peekable<I>);

impl<I: Iterator> TokenStream<I> {
    fn new(iterator: I) -> Self {
        Self(iterator.peekable())
    }
}

impl<I: Iterator<Item = TokenAndPos>> Stream for TokenStream<I> {
    fn peek(&mut self) -> Option<&TokenAndPos> {
        self.0.peek()
    }

    fn next(&mut self) -> Option<TokenAndPos> {
        self.0.next()
    }
}

pub fn new<R: Reader>(reader: R) -> impl Stream {
    TokenStream::new(TokenIterator::new(reader))
}
