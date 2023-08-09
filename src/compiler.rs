use core::fmt;

use crate::{
    opcode::*,
    push::{PushByte, PushData},
    token::*,
};

pub trait Stream {
    fn peek(&mut self) -> Option<&TokenAndPos>;
    fn next(&mut self) -> Option<TokenAndPos>;
}

pub enum Message {
    Owned(Box<str>),
    Static(&'static str),
}

impl From<&'static str> for Message {
    fn from(value: &'static str) -> Self {
        Message::Static(value)
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Message::Owned(value.into_boxed_str())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Owned(s) => write!(f, "{s}"),
            Message::Static(s) => write!(f, "{s}"),
        }
    }
}

pub struct CompileError {
    pub message: Message,
    pub pos: Pos,
}

pub type CompileResult = Result<(), CompileError>;

fn primary<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    match stream.next() {
        Some(token_and_pos) => match token_and_pos.token {
            Token::Integer(value) => {
                builder.push_byte(LDI);
                builder.push_data(value);
                Ok(())
            }
            Token::Real(value) => {
                builder.push_byte(LDR);
                builder.push_data(value);
                Ok(())
            }
            Token::Single(c) => Err(CompileError {
                message: format!("Expected value, found character {}.", c as char).into(),
                pos: token_and_pos.pos,
            }),
        },
        None => Err(CompileError {
            message: "Unexpected end of code.".into(),
            pos: 0..0,
        }),
    }
}

fn binary_helper<S: Stream, P: PushByte, N, M>(
    stream: &mut S,
    builder: &mut P,
    next: N,
    mapper: M,
) -> CompileResult
where
    N: Fn(&mut S, &mut P) -> CompileResult,
    M: Fn(&Token) -> Option<u8>,
{
    next(stream, builder)?;
    while let Some(token_and_pos) = stream.peek() {
        if let Some(opcode) = mapper(&token_and_pos.token) {
            let _pos = token_and_pos.pos.clone();
            stream.next();
            next(stream, builder)?;
            builder.push_byte(opcode);
        } else {
            break;
        }
    }
    Ok(())
}

fn factor<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    binary_helper(stream, builder, primary, |token| match token {
        Token::Single(b'*') => Some(MUL),
        Token::Single(b'/') => Some(DIV),
        _ => None,
    })
}

fn term<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    binary_helper(stream, builder, factor, |token| match token {
        Token::Single(b'+') => Some(ADD),
        Token::Single(b'-') => Some(SUB),
        _ => None,
    })
}

fn binary<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    term(stream, builder)
}

fn expression<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    binary(stream, builder)
}

pub fn compile<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    if stream.peek().is_some() {
        expression(stream, builder)?;
    }

    match stream.next() {
        Some(token_and_pos) => match token_and_pos.token {
            Token::Integer(value) => Err(CompileError {
                message: format!("Expected end of code, found integer '{value}'.").into(),
                pos: token_and_pos.pos,
            }),
            Token::Real(value) => Err(CompileError {
                message: format!("Expected end of code, found real '{value}'.").into(),
                pos: token_and_pos.pos,
            }),
            Token::Single(c) => Err(CompileError {
                message: format!("Expected end of code, found character '{}'.", c as char).into(),
                pos: token_and_pos.pos,
            }),
        },
        None => {
            builder.push_byte(END);
            Ok(())
        }
    }
}
