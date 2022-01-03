use crate::vm::*;

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

fn char_to_token(c: char) -> Option<Token> {
    match c {
        '+' => Some(Token::ADD),
        '-' => Some(Token::SUB),
        '<' => Some(Token::LEFT),
        '>' => Some(Token::RIGHT),
        '[' => Some(Token::BEGIN),
        ']' => Some(Token::END),
        ',' => Some(Token::INPUT),
        '.' => Some(Token::OUTPUT),
        _ => None,
    }
}

fn text_to_tokens(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for i in text.chars() {
        match char_to_token(i) {
            Some(token) => tokens.push(token),
            None => {}
        }
    }
    return tokens;
}

struct BracketPair {
    begin: usize,
    end: usize,
}

fn pair_brackets(tokens: &mut Vec<Token>) -> Vec<BracketPair> {
    let mut bracket_pairs: Vec<BracketPair> = Vec::new();
    let mut begin_brackets: Vec<usize> = Vec::new();
    for (i, v) in tokens.iter().enumerate() {
        match v {
            Token::BEGIN => begin_brackets.push(i),
            Token::END => bracket_pairs.push(BracketPair {
                begin: begin_brackets.pop().expect("_"),
                end: i,
            }),
            _ => {}
        }
    }
    bracket_pairs.reverse();
    return bracket_pairs;
}

fn zero_valued(op: Ops) -> Instruction {
    Instruction { op: op, value: 0 }
}

fn jz_instruction(index: usize, bracket_pairs: &mut Vec<BracketPair>) -> Instruction {
    Instruction {
        op: Ops::JZ,
        value: bracket_pairs
            .iter()
            .find(|&p| p.begin == index)
            .expect("_")
            .end,
    }
}

fn jmp_instruction(bracket_pairs: &mut Vec<BracketPair>) -> Instruction {
    Instruction {
        op: Ops::JMP,
        value: bracket_pairs.pop().expect("_").begin,
    }
}

fn token_to_instruction(
    index: usize,
    token: &Token,
    bracket_pairs: &mut Vec<BracketPair>,
) -> Instruction {
    match token {
        Token::ADD => zero_valued(Ops::ADD),
        Token::SUB => zero_valued(Ops::SUB),
        Token::LEFT => zero_valued(Ops::LEFT),
        Token::RIGHT => zero_valued(Ops::RIGHT),
        Token::INPUT => zero_valued(Ops::INPUT),
        Token::OUTPUT => zero_valued(Ops::OUTPUT),
        Token::BEGIN => jz_instruction(index, bracket_pairs),
        Token::END => jmp_instruction(bracket_pairs),
    }
}

fn tokens_to_program(tokens: &mut Vec<Token>) -> Vec<Instruction> {
    let bracket_pairs = &mut pair_brackets(tokens);
    let mut program: Vec<Instruction> = tokens
        .iter()
        .enumerate()
        .map(|(index, token)| token_to_instruction(index, token, bracket_pairs))
        .collect();
    program.push(zero_valued(Ops::EXIT));
    return program;
}

pub fn parse(text: String) -> Vec<Instruction> {
    let tokens = &mut text_to_tokens(text);
    return tokens_to_program(tokens);
}
