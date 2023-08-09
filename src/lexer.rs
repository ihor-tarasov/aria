use crate::token::*;

pub trait Reader {
    fn current(&mut self) -> Option<u8>;
    fn advance(&mut self);
    fn offset(&self) -> usize;
}

fn lex_token<R: Reader>(reader: &mut R) -> Option<Token> {
    let c = reader.current()?;
    reader.advance();
    Some(match c {
        b'0'..=b'9' => Token::Integer((c - b'0') as i64),
        _ => Token::Single(c),
    })
}

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\r' || c == b'\n'
}

fn skip_whitespaces<R: Reader>(reader: &mut R) {
    while let Some(c) = reader.current() {
        if is_whitespace(c) {
            reader.advance();
        } else {
            break;
        }
    }
}

pub fn lex<R: Reader>(reader: &mut R) -> Option<TokenAndPos> {
    skip_whitespaces(reader);
    let start = reader.offset();
    let token = lex_token(reader)?;
    let end = reader.offset();
    Some(TokenAndPos {
        token,
        pos: start..end,
    })
}
