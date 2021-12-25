
use crate::vm::*;

#[derive(PartialEq, Debug)]
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

struct BracePair {
    begin: usize,
    end: usize,
}

fn get_brace_pairs(tokens: &mut Vec<Token>) -> Vec<BracePair> {
    let mut brace_pairs: Vec<BracePair> = Vec::new();
    let mut begin_braces: Vec<usize> = Vec::new();
    for (i, v) in tokens.iter().enumerate() {
        match v {
            Token::BEGIN => begin_braces.push(i),
            Token::END => brace_pairs.push(BracePair {
                begin: begin_braces.pop().expect("_"),
                end: i,
            }),
            _ => {}
        }
    }
    brace_pairs.reverse();
    return brace_pairs;
}

fn get_ops(tokens: &mut Vec<Token>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut brace_pairs = get_brace_pairs(tokens);
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::ADD     => {program.push(Instruction {op: Ops::ADD,     value: 0})},
            Token::SUB     => {program.push(Instruction {op: Ops::SUB,     value: 0})},
            Token::LEFT    => {program.push(Instruction {op: Ops::LEFT,    value: 0})},
            Token::RIGHT   => {program.push(Instruction {op: Ops::RIGHT,   value: 0})},
            Token::INPUT   => {program.push(Instruction {op: Ops::INPUT,   value: 0})},
            Token::OUTPUT  => {program.push(Instruction {op: Ops::OUTPUT,  value: 0})},        
            Token::BEGIN   => {
                program.push(Instruction {
                    op: Ops::JZ,
                    value: brace_pairs.iter().find(|&p| p.begin == i).expect(&format!("bruh moment at {}, {:?}", i, token)).end
                })
            },
            Token::END     => {
                program.push(Instruction {
                    op: Ops::JMP,
                    value: brace_pairs.pop().expect("_").begin
            })
            },
        }
    }
    program.push(Instruction {op: Ops::EXIT, value: 0});
    return program;
}

pub fn parse(text: String) -> Vec<Instruction> {
    let tokens = &mut tokenize(text);
    let brace_pairs = get_brace_pairs(tokens);
    for (i, v) in brace_pairs.iter().enumerate() {
        println!("{}: {} -> {}", i, v.begin, v.end);
    };
    return get_ops(tokens);
}


