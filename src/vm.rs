use crate::{
    get::{GetByte, GetData},
    opcode::*,
    state::*,
    state::{VMError, VMResult},
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
            state.push(value)?;
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
        _ => Err(VMError::UnknownInstruction),
    }
}

pub fn run<S: Stack, G: GetByte>(state: &mut State<S>, program: &G) -> VMResult<i64> {
    while step(state, program)? {}
    state.pop()
}
