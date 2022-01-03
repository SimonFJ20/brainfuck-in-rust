mod parser;
mod vm;
mod dump;

use crate::dump::dump;
use crate::vm::Context;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn filename_from_args() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Not enough args");
        exit(1);
    }
    return args[1].clone();
}

fn file_content(filename: String) -> String {
    return read_to_string(filename).expect("Something went wrong reading the file");
}

fn dump_arg_exists() -> bool {
    let args: Vec<String> = args().collect();
    if args.len() < 3 || args[2] != "--dump" {
        false
    } else {
        true
    }
}

fn main() {
    let text = file_content(filename_from_args());
    let program = parser::parse(text);
    let ctx = &mut Context {
        program: program,
        pc: 0,
        stack: Vec::from([0]),
        sp: 0,
    };
    vm::eval(ctx);
    if dump_arg_exists() {
        dump(ctx);
    }
}
