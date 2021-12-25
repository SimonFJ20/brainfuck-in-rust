
mod parser;
mod vm;

use std::env::args;
use std::process::exit;
use std::fs::{read_to_string, write};

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

fn dump_instructions(program: &Vec<vm::Instruction>) {
    let mut text = String::new();
    let mut depth = 0;
    for instruction in program {
        text.push_str(&String::from("  ").repeat(depth));
        match instruction.op {
            vm::Ops::EXIT    => text.push_str("exit"),
            vm::Ops::LEFT    => text.push_str("left"),
            vm::Ops::RIGHT   => text.push_str("right"),
            vm::Ops::ADD     => text.push_str("add"),
            vm::Ops::SUB     => text.push_str("sub"),
            vm::Ops::INPUT   => text.push_str("input"),
            vm::Ops::OUTPUT  => text.push_str("output"),
            vm::Ops::JZ => {
                depth += 1;
                text.push_str("jz");
            },
            vm::Ops::JMP => {
                depth -= 1;
                text.push_str("jmp");
            },
        }
    }
    write("instructions.txt", text).expect("_");
}

fn main() {
    let text = file_content(filename_from_args());
    let program = parser::parse(text);
    // for (i, v) in program.iter().enumerate() {
    //     println!("{}: {:?}", i, v);
    // }
    dump_instructions(&program);
    vm::eval(program);
}
