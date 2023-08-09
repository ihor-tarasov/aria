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
            Token::Single(c) => Err(CompileError {
                message: Message::Owned(
                    format!("Expected value, found character {}.", c as char).into_boxed_str(),
                ),
                pos: token_and_pos.pos,
            }),
        },
        None => Err(CompileError {
            message: Message::Static("Unexpected end of code."),
            pos: 0..0,
        }),
    }
}

fn factor<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    primary(stream, builder)?;
    while let Some(token) = stream.peek() {
        if token.token == Token::Single(b'*') {
            let _pos = token.pos.clone();
            stream.next();
            primary(stream, builder)?;
            builder.push_byte(MUL);
        } else {
            break;
        }
    }
    Ok(())
}

fn term<S: Stream, P: PushByte>(stream: &mut S, builder: &mut P) -> CompileResult {
    factor(stream, builder)?;
    while let Some(token) = stream.peek() {
        if token.token == Token::Single(b'+') {
            let _pos = token.pos.clone();
            stream.next();
            factor(stream, builder)?;
            builder.push_byte(ADD);
        } else {
            break;
        }
    }
    Ok(())
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
                message: Message::Owned(
                    format!("Expected end of code, found integer '{value}'.").into_boxed_str(),
                ),
                pos: token_and_pos.pos,
            }),
            Token::Single(c) => Err(CompileError {
                message: Message::Owned(
                    format!("Expected end of code, found character '{}'.", c as char)
                        .into_boxed_str(),
                ),
                pos: token_and_pos.pos,
            }),
        },
        None => {
            builder.push_byte(END);
            Ok(())
        }
    }
}
