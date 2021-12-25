
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

struct BracePair {
    begin: usize,
    end: usize,
}

fn get_brace_pairs(tokens: Vec<Token>) -> Vec<BracePair> {
    let brace_pairs: Vec<BracePair> = Vec::new();
    let begin_braces: Vec<usize> = Vec::new();
    for (i, v) in tokens.iter().enumerate() {
        match v {
            Token::BEGIN => begin_braces.push(i),
            Token::END => brace_pairs.push(BracePair {
                end: i,
                begin: match begin_braces.pop() {
                    Some(v) => v,
                    None => { panic!("punjabi no begin_brace"); }
                },
            }),
            _ => {}
        }
    }
    return brace_pairs;
}

fn get_ops(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut brace_pairs = get_brace_pairs(tokens);
    for token in tokens {
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
                    value: brace_pairs.search()
                })
            },
            Token::END     => {program.push(Instruction {op: Ops::JMP,     value: match begin_brackets.pop() {
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


