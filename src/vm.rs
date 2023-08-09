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
        ADD => state.single(State::addict),
        MUL => state.single(State::multiply),
        SUB => state.single(State::subtract),
        DIV => state.single(State::divide),
        MOD => state.single(State::module),
        LS => state.single(State::less),
        GR => state.single(State::greater),
        LE => state.single(State::less_equals),
        GE => state.single(State::greater_equals),
        EQ => state.single(State::equals),
        NE => state.single(State::not_equals),
        AND => state.single(State::and),
        OR => state.single(State::or),
        XOR => state.single(State::xor),
        SHL => state.single(State::shift_left),
        SHR => state.single(State::shift_right),
        _ => Err(VMError::UnknownInstruction),
    }
}

pub fn run<S: Stack, G: GetByte>(state: &mut State<S>, program: &G) -> VMResult<Value> {
    while step(state, program)? {}
    state.pop()
}
