use crate::optimizer::Optimized;
use std::io::prelude::*;
use std::io::stdin;
use std::process::exit;

#[derive(PartialEq, Clone, Copy)]
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

#[derive(Clone, Copy)] 
pub struct Instruction {
    pub op: Ops,
    pub offset: usize,
    pub value: u8,
}

pub type Program = Vec<Instruction>;

pub struct Context {
    pub program: Optimized<Program>,
    pub pc: usize,
    pub stack: Vec<u8>,
    pub sp: usize,
}

impl Context {
    pub fn new(program: Optimized<Program>) -> Self {
        Context {
            program: program,
            pc: 0,
            stack: Vec::from([0]),
            sp: 0,
        }
    }
}

fn left(ctx: &mut Context) {
    if ctx.sp == 0 {
        panic!("the stack pointer must not go below zero")
    } else {
        ctx.sp -= 1
    }
}

fn right(ctx: &mut Context) {
    ctx.sp += 1;
    if ctx.sp >= ctx.stack.len() {
        ctx.stack.push(0)
    }
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

// fn add_more(ctx: &mut Context, ins: Instruction) {
//     let mut res: usize = ctx.stack[ctx.sp].into();
//     res += ins.offset;
//     if res > 255 {
//         res %= 256;
//     }
//     ctx.stack[ctx.sp] = res.try_into().expect("error converting usize to u8");
// }

fn jz(ctx: &mut Context) {
    if ctx.stack[ctx.sp] == 0 {
        ctx.pc = ctx.program[ctx.pc].offset
    }
}

fn jmp(ctx: &mut Context) {
    ctx.pc = ctx.program[ctx.pc].offset - 1;
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

pub fn eval(ctx: &mut Context) {
    run(ctx);
}
