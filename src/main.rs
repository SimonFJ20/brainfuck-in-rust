
use std::process::exit;
use std::fs::read_to_string;
use std::env::args;
use std::io::prelude::*;
use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Operation {
    EOF,
    INCR,
    DECR,
    LEFT,
    RIGHT,
    BEGIN,
    END,
    OUTPUT,
    INPUT,
}

struct Context {
    ram: [u8; 30000],
    call_stack: Vec<usize>,
    program: [Operation; 10000],
    sp: usize,
    pc: usize,
}


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

fn load_program(ctx: &mut Context, text: String) {
    for i in text.chars() {
        match i {
            '+' => {ctx.program[ctx.pc] = Operation::INCR;      ctx.pc += 1},
            '-' => {ctx.program[ctx.pc] = Operation::DECR;      ctx.pc += 1},
            '<' => {ctx.program[ctx.pc] = Operation::LEFT;      ctx.pc += 1},
            '>' => {ctx.program[ctx.pc] = Operation::RIGHT;     ctx.pc += 1},
            '[' => {ctx.program[ctx.pc] = Operation::BEGIN;     ctx.pc += 1},
            ']' => {ctx.program[ctx.pc] = Operation::END;       ctx.pc += 1},
            '.' => {ctx.program[ctx.pc] = Operation::OUTPUT;    ctx.pc += 1},
            ',' => {ctx.program[ctx.pc] = Operation::INPUT;     ctx.pc += 1},
            _ => {}
        }
    }
    ctx.program[ctx.pc] = Operation::EOF;
    ctx.pc = 0;
}

fn incr(ctx: &mut Context) {
    if ctx.ram[ctx.sp] == 255 {
        ctx.ram[ctx.sp] = 0;
    } else {
        ctx.ram[ctx.sp] += 1;
    }
}

fn decr(ctx: &mut Context) {
    if ctx.ram[ctx.sp] == 0 {
        ctx.ram[ctx.sp] = 255;
    } else {
        ctx.ram[ctx.sp] -= 1;
    }
}

fn left(ctx: &mut Context) {
    if ctx.sp == 0 {
        ctx.sp = ctx.program.len();
    } else {
        ctx.sp -= 1;
    }
}

fn right(ctx: &mut Context) {
    if ctx.sp == ctx.program.len() {
        ctx.sp = 0;
    } else {
        ctx.sp += 1;
    }
}

fn begin(ctx: &mut Context) {
    ctx.call_stack.push(ctx.pc - 1)
}

fn end(ctx: &mut Context) {
    if ctx.ram[ctx.sp] != 0 {
        match ctx.call_stack.pop() {
            Some(v) => ctx.pc = v,
            None => {
                println!("Hit unexpected loop ending at");
                exit(1);
            }
        }
    }
    
}

fn eof(ctx: &mut Context) {
    if ctx.ram[0] == 0 {
        exit(0)
    } else {
        println!("Exited with error code {}", ctx.ram[0]);
        exit(ctx.ram[0].into());
    }
}

fn output(ctx: &mut Context) {
    print!("{}", match String::from_utf8(Vec::from([ctx.ram[ctx.sp]])) {
        Ok(v) => v,
        Err(e) => {
            println!("Error printing to STDOUT: {}", e);
            exit(1);
        }
    })
}

fn input(ctx: &mut Context) {
    let buffer: &mut [u8] = &mut [0, 1];
    let result = io::stdin().take(1).read(buffer);
    match result {
        Ok(_) => {},
        Err(_) => {
            println!("Error retrieving from STDIN");
            exit(1);
        }
    }
    ctx.ram[ctx.sp] = buffer[0]
}

fn run(ctx: &mut Context) {
    while ctx.program[ctx.pc] != Operation::EOF {
        println!("\tprogram[{}] = {:?}\tram[{}] = {}", ctx.pc, ctx.program[ctx.pc], ctx.sp, ctx.ram[ctx.sp]);
        match ctx.program[ctx.pc] {
            Operation::INCR     => incr(ctx),
            Operation::DECR     => decr(ctx),
            Operation::LEFT     => left(ctx),
            Operation::RIGHT    => right(ctx),
            Operation::BEGIN    => begin(ctx),
            Operation::END      => end(ctx),
            Operation::EOF      => eof(ctx),
            Operation::OUTPUT   => output(ctx),
            Operation::INPUT    => input(ctx),
        };
        ctx.pc += 1;
    }
}

fn main() {
    let ctx = &mut Context {
        ram: [0; 30000],
        program: [Operation::EOF; 10000],
        call_stack: Vec::new(),
        sp: 0,
        pc: 0,
    };
    let filename = filename_from_args();
    let file_content = file_content(filename);
    load_program(ctx, file_content);
    run(ctx);
}
