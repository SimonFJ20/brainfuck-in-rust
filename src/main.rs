
mod parser;
mod vm;

use std::env::args;
use std::process::exit;
use std::fs::read_to_string;

fn filename_from_args() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Not enough args");
        exit(1);
    }
    return args[1].clone();
}

fn file_content(filename: String) -> String {
    return read_to_string(filename)
        .expect("Something went wrong reading the file");
}


fn main() {
    let text = file_content(filename_from_args());
    let program = parser::parse(text);
    for (i, v) in program.iter().enumerate() {
        println!("{}: {:?}", i, v);
    }
    // vm::eval(program);
}
