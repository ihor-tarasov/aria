use std::io::Write;

use tpc::{
    compiler::{self, CompileError},
    impls::{
        data_stack, slice_reader, static_data, token_stream,
        vec_push::{self},
    },
    line,
    push::IntoGetByte,
    state,
    value::Value,
    vm,
};

fn print_error(error: CompileError, slice: &[u8]) {
    let line_info = line::create(slice_reader::new(slice), error.pos.start);
    println!("In file: \"stdin\", line: {}", line_info.number);
    line::print_line(slice_reader::new(slice.as_ref()), line_info.start);
    line::mark_range(line_info.start, error.pos);
    println!("{}", error.message);
}

fn run_slice<S: AsRef<[u8]>>(slice: S) -> Option<Value> {
    let reader = slice_reader::new(slice.as_ref());
    let mut stream = token_stream::new(reader);
    let mut builder = vec_push::new();
    match compiler::compile(&mut stream, &mut builder) {
        Ok(_) => (),
        Err(error) => {
            print_error(error, slice.as_ref());
            return None;
        }
    }
    let program = builder.into_get_byte();
    let mut state = state::State::new(data_stack::new(static_data::new::<256>()));
    match vm::run(&mut state, &program) {
        Ok(value) => Some(value),
        Err(error) => {
            if let Some(message) = state.message {
                println!("Runtime error: {message}");
            } else {
                println!("Runtime error: {error}");
            }
            None
        }
    }
}

fn main() {
    let mut line = String::new();
    loop {
        line.clear();
        print!("-> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        if let Some(value) = run_slice(&line) {
            println!("{value}");
        }
    }
}

#[test]
fn base_test() {
    assert_eq!(run_slice("2 + 2 * 2"), Some(Value::Integer(6)))
}
