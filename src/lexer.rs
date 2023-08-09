use crate::token::*;

pub trait Reader {
    fn current(&mut self) -> Option<u8>;
    fn advance(&mut self);
    fn offset(&self) -> usize;
}

fn lex_number<R: Reader>(reader: &mut R, c: u8) -> Token {
    let mut result = (c - b'0') as i64;
    let mut digits_after_dot = 0u32;
    while let Some(c) = reader.current() {
        if c.is_ascii_digit() {
            result = result * 10 + (c - b'0') as i64;
            reader.advance();
            if digits_after_dot != 0 {
                digits_after_dot += 1;
            }
        } else if c == b'.' {
            if digits_after_dot == 0 {
                digits_after_dot = 1;
                reader.advance();
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if digits_after_dot == 0 {
        Token::Integer(result)
    } else {
        Token::Real(result as f64 / 10u64.pow(digits_after_dot - 1) as f64)
    }
}

fn lex_token<R: Reader>(reader: &mut R) -> Option<Token> {
    let c = reader.current()?;
    reader.advance();
    Some(match c {
        b'0'..=b'9' => lex_number(reader, c),
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
