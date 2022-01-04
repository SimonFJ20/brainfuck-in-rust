
use std::collections::HashMap;
use crate::vm::{Instruction, Ops, Program};

fn push_unless_repeated(mut program: Program, op: Ops, v: Instruction) -> Program {
    let last = program.len() - 1;
    if program.len() > 0 && program[last].op == op {
        program[last].offset += 1;
    } else {
        program.push(v);
    }
    return program;
}

type RepeatOptimized<T> = T;
fn repeat_optimize(program: Program) -> RepeatOptimized<Program> {
    program
        .iter()
        .fold(Vec::new(), |mut optimized_program, &v| match v.op {
            // Ops::ADD => push_unless_repeated(optimized_program, Ops::ADD, v),
            // Ops::SUB => push_unless_repeated(optimized_program, Ops::SUB, v),
            Ops::LEFT => push_unless_repeated(optimized_program, Ops::LEFT, v),
            Ops::RIGHT => push_unless_repeated(optimized_program, Ops::RIGHT, v),
            _ => {
                optimized_program.push(v);
                optimized_program
            }
        })
}

fn push_unless_contra(mut program: Program, counter: Ops, mut v: Instruction) -> Program {
    let last = program.len() - 1;
    if program.len() > 0 && program[last].op == counter {
        if program[last].value == v.value {
            program.pop();
        } else if program[last].value < v.value {
            program.pop();
            v.value -= program[last].value;
            program.push(v);
        } else {
            program[last].value -= v.value;
        }
    }
    return program;
}

type ContraOptimized<T> = T;
fn contra_optimize(program: Program) -> ContraOptimized<Program> {
    program
        .iter()
        .fold(Vec::new(), |mut optimized_program, &v| match v.op {
            // Ops::ADD => push_unless_contra(optimized_program, Ops::SUB, v),
            // Ops::SUB => push_unless_contra(optimized_program, Ops::ADD, v),
            Ops::LEFT => push_unless_contra(optimized_program, Ops::RIGHT, v),
            Ops::RIGHT => push_unless_contra(optimized_program, Ops::LEFT, v),
            _ => {
                optimized_program.push(v);
                optimized_program
            }
        })
}

type JumpMap = HashMap<usize, usize>;
fn map_jumps(program: &Program) -> JumpMap {
    let mut map = HashMap::new();
    for v in program {
        if v.op == Ops::JZ {
            map.insert(v.offset, program[v.offset].offset);
        }
    }
    return map;
}

type ShiftCorrected<T=Program> = T;
fn shift_correct(program: Program, jump_map: JumpMap) -> ShiftCorrected<Program> {
    program
        .iter()
        .map(|&v| match v.op {
            Ops::JZ => {v},
            Ops::JMP => {v},
            _ => v
        }).collect()
}

pub type Optimized<T> = ShiftCorrected<ContraOptimized<RepeatOptimized<T>>>;
pub fn optimize(program: Program) -> Optimized<Program> {
    let jump_map = map_jumps(&program);
    return shift_correct(contra_optimize(repeat_optimize(program)), jump_map);
}
