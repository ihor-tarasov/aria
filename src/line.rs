use crate::lexer::Reader;
use core::ops::Range;

pub struct LineInfo {
    pub start: usize,
    pub number: usize,
}

pub fn create<R: Reader>(mut reader: R, start: usize) -> LineInfo {
    let mut line_number = 1;
    let mut line_start = 0;
    while let Some(c) = reader.current() {
        if reader.offset() == start {
            break;
        }

        reader.advance();

        if c == b'\n' {
            line_number += 1;
            line_start = reader.offset();
        }
    }
    LineInfo {
        start: line_start,
        number: line_number,
    }
}

pub fn print_line<R: Reader>(mut reader: R, start: usize) {
    while let Some(_) = reader.current() {
        if reader.offset() < start {
            reader.advance();
        } else {
            break;
        }
    }

    let mut line = Vec::new();

    while let Some(c) = reader.current() {
        if c != b'\n' && c != b'\r' {
            line.push(c);
            reader.advance();
        } else {
            break;
        }
    }

    let line = String::from_utf8(line).unwrap();

    println!("{line}");
}

pub fn mark_range(line_start: usize, range: Range<usize>) {
    for _ in line_start..range.start {
        print!(" ");
    }
    for _ in range {
        print!("^");
    }
    println!()
}
