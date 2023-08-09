use crate::{
    get::{GetByte, GetData},
    opcode::*,
    state::*,
    state::{VMError, VMResult},
    value::Value,
};

fn step<S: Stack, G: GetByte>(state: &mut State<S>, program: &G) -> VMResult<bool> {
    let opcode = program
        .get_byte(state.program_counter)
        .ok_or(VMError::OpcodeFetch)?;
    match opcode {
        END => Ok(false),
        LDI => {
            let value = program
                .get_data(state.program_counter + 1)
                .ok_or(VMError::OpcodeFetch)?;
            state.push(Value::Integer(value))?;
            state.program_counter += 1 + core::mem::size_of_val(&value);
            Ok(true)
        }
        LDR => {
            let value = program
                .get_data(state.program_counter + 1)
                .ok_or(VMError::OpcodeFetch)?;
            state.push(Value::Real(value))?;
            state.program_counter += 1 + core::mem::size_of_val(&value);
            Ok(true)
        }
        ADD => {
            state.addict()?;
            state.program_counter += 1;
            Ok(true)
        }
        MUL => {
            state.multiply()?;
            state.program_counter += 1;
            Ok(true)
        }
        SUB => {
            state.subtract()?;
            state.program_counter += 1;
            Ok(true)
        }
        DIV => {
            state.divide()?;
            state.program_counter += 1;
            Ok(true)
        }
        MOD => {
            state.module()?;
            state.program_counter += 1;
            Ok(true)
        }
        LS => {
            state.less()?;
            state.program_counter += 1;
            Ok(true)
        }
        GR => {
            state.greater()?;
            state.program_counter += 1;
            Ok(true)
        }
        LE => {
            state.less_equals()?;
            state.program_counter += 1;
            Ok(true)
        }
        GE => {
            state.greater_equals()?;
            state.program_counter += 1;
            Ok(true)
        }
        EQ => {
            state.equals()?;
            state.program_counter += 1;
            Ok(true)
        }
        NE => {
            state.not_equals()?;
            state.program_counter += 1;
            Ok(true)
        }
        _ => Err(VMError::UnknownInstruction),
    }
}

pub fn run<S: Stack, G: GetByte>(state: &mut State<S>, program: &G) -> VMResult<Value> {
    while step(state, program)? {}
    state.pop()
}
