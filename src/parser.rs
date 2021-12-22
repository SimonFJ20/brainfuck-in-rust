
use crate::vm::*;

#[derive(PartialEq)]
enum Token {
    ADD,
    SUB,
    LEFT,
    RIGHT,
    BEGIN,
    END,
    INPUT,
    OUTPUT,
}

fn tokenize(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for i in text.chars() {
        match i {
            '+' => {tokens.push(Token::ADD)},
            '-' => {tokens.push(Token::SUB)},
            '<' => {tokens.push(Token::LEFT)},
            '>' => {tokens.push(Token::RIGHT)},
            '[' => {tokens.push(Token::BEGIN)},
            ']' => {tokens.push(Token::END)},
            ',' => {tokens.push(Token::INPUT)},
            '.' => {tokens.push(Token::OUTPUT)},
            _ => {}
        }
    }
    return tokens;
}

fn get_bracket(program: & Vec<Token>, start: u32) {
    let mut pos = start;
    let mut nested = 0;
    for n in program.iter().skip(start.try_into().unwrap()) {
        pos += 1;
        if *n == Token::BEGIN {
            nested += 1;
        } else if *n == Token::END {
            if nested == 0 {
                start,
                pos,
            }
            nested -= 1;
        }
    }
}


fn get_begin_brackets(program: & Vec<Token>) -> Vec<usize> {
    let mut brackets: Vec<usize> = Vec::new();
    for (i, v) in program.iter().enumerate() {
        match v {
            Token::BEGIN => brackets.push(i),
            _ => {}
        }
    }
    return brackets;
}

fn get_end_brackets(program: & Vec<Token>) -> Vec<usize> {
    let mut brackets: Vec<usize> = Vec::new();
    for (i, v) in program.iter().enumerate().rev() {
        match v {
            Token::END => brackets.push(i),
            _ => {}
        }
    }
    return brackets;
}

fn get_ops(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut begin_brackets = get_begin_brackets(&tokens);
    let mut end_brackets = get_end_brackets(&tokens);
    for token in tokens {
        match token {
            Token::ADD     => {program.push(Instruction {op: Ops::ADD,     value: 0})},
            Token::SUB     => {program.push(Instruction {op: Ops::SUB,     value: 0})},
            Token::LEFT    => {program.push(Instruction {op: Ops::LEFT,    value: 0})},
            Token::RIGHT   => {program.push(Instruction {op: Ops::RIGHT,   value: 0})},
            Token::INPUT   => {program.push(Instruction {op: Ops::INPUT,   value: 0})},
            Token::OUTPUT  => {program.push(Instruction {op: Ops::OUTPUT,  value: 0})},        
            Token::BEGIN   => {program.push(Instruction {op: Ops::JZ,      value: match end_brackets.pop() {
                Some(v) => {v},
                None => {panic!("punjabi no begin brace")}
            }})},
            Token::END     => {program.push(Instruction {op: Ops::JNZ,     value: match begin_brackets.pop() {
                Some(v) => {v},
                None => {panic!("punjabi no end brace")}
            }})},
        }
    }
    program.push(Instruction {op: Ops::EXIT, value: 0});
    return program;
}

pub fn parse(text: String) -> Vec<Instruction> {
    let tokens = tokenize(text);
    return get_ops(tokens);
}


