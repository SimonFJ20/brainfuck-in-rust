use std::io::prelude::*;
use std::io::stdin;
use std::process::exit;

#[derive(PartialEq)]
pub enum Ops {
    EXIT,
    LEFT,
    RIGHT,
    ADD,
    SUB,
    INPUT,
    OUTPUT,
    JZ,
    JMP,
}

pub struct Instruction {
    pub op: Ops,
    pub value: usize,
}

pub struct Context {
    program: Vec<Instruction>,
    pc: usize,
    stack: Vec<u8>,
    sp: usize,
}

fn add(ctx: &mut Context) {
    if ctx.stack[ctx.sp] == 255 {
        ctx.stack[ctx.sp] = 0;
    } else {
        ctx.stack[ctx.sp] += 1;
    }
}

fn sub(ctx: &mut Context) {
    if ctx.stack[ctx.sp] == 0 {
        ctx.stack[ctx.sp] = 255;
    } else {
        ctx.stack[ctx.sp] -= 1;
    }
}

fn left(ctx: &mut Context) {
    if ctx.sp == 0 {
        ctx.sp = ctx.program.len() - 1
    } else {
        ctx.sp -= 1
    }
}

fn right(ctx: &mut Context) {
    ctx.sp += 1;
    if ctx.sp >= ctx.stack.len() - 1 {
        ctx.stack.push(0)
    }
}

fn jz(ctx: &mut Context) {
    if ctx.stack[ctx.sp] == 0 {
        ctx.pc = ctx.program[ctx.pc].value
    }
}

fn jmp(ctx: &mut Context) {
    ctx.pc = ctx.program[ctx.pc].value - 1;
}

fn input(ctx: &mut Context) {
    let buffer: &mut [u8] = &mut [0, 1];
    let result = stdin().take(1).read(buffer);
    match result {
        Ok(_) => {}
        Err(_) => {
            println!("Error retrieving from STDIN");
            exit(1);
        }
    }
    ctx.stack[ctx.sp] = buffer[0];
}

fn output(ctx: &mut Context) {
    print!("{}", ctx.stack[ctx.sp] as char);
}

fn run(ctx: &mut Context) {
    while ctx.program[ctx.pc].op != Ops::EXIT {
        match ctx.program[ctx.pc].op {
            Ops::EXIT => break,
            Ops::LEFT => left(ctx),
            Ops::RIGHT => right(ctx),
            Ops::ADD => add(ctx),
            Ops::SUB => sub(ctx),
            Ops::INPUT => input(ctx),
            Ops::OUTPUT => output(ctx),
            Ops::JZ => jz(ctx),
            Ops::JMP => jmp(ctx),
        }
        ctx.pc += 1;
    }
}

pub fn eval(program: Vec<Instruction>) {
    let ctx = &mut Context {
        program: program,
        pc: 0,
        stack: Vec::from([0]),
        sp: 0,
    };
    run(ctx);
}
