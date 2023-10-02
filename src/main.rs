extern crate core;

use crate::ParserState::NoneState;

#[derive(Debug)]
enum Token {
    Text {t: String},
    EOL,
}

enum ParserState {
    Commenting,
    NoneState,
    Construct{i: Instruction},
}

enum Modifier {
    A, B, AB, BA, F, X, I
}

enum Mode {
    IMMEDIATE,
    DIRECT,
    INDIRECT,
    DECREMENT,
    INCREMENT,
}

enum OpCode {
    DAT, MOV, ADD, SUB, MUL, DIV, MOD,
    JMP, JMZ, JMN, DJN, SPL, SLT, CMP,
    SEQ, SNE, NOP,
}

struct Instruction {
    opcode: OpCode,
    modifier: Modifier,
    a_mode: Mode,
    a_number: usize,
    b_mode: Mode,
    b_number: usize,
}

fn main() {
    let warrior_1 : String =
        "; This is a comment.\
        \nmov 1\
        \ntest 2 ;Test comment 2.\
        \n;Test comment 3. \
        \nmulti 1 2"
            .to_string();
    println!("{}",warrior_1);
    let mut agg_tokens: Vec<Token> = tokenise_input(warrior_1);

    println!("{:?}",agg_tokens);
    let instructions: Vec<Instruction> = parse_instructions(agg_tokens);

    let warrior_2: String =
        "; This is a comment.\
        \nmov 2\
        \ntest 2 ;Test comment 2.\
        \n;Test comment 3. \
        \nmulti 1 2"
            .to_string();

}

fn parse_instructions(input: Vec<Token>) -> Vec<Instruction> {
    let mut state: ParserState = ParserState::NoneState;
    let mut instructions: Vec<Instruction> = Vec::new();
    for token in input {
        match token {
            Token::Text {t} => {
                match state {
                    ParserState::Commenting => {}
                    NoneState => {}
                    ParserState::Construct { .. } => {}
                }
            }
            Token::EOL => {
                state = ParserState::NoneState;
            }
        }
    }
    return instructions;
}

fn tokenise_input(input: String) -> Vec<Token> {
    let input_line: Vec<&str> = input.split("\n").collect();
    let mut agg_tokens: Vec<Token> = Vec::new();
    for line in input_line {
        let split_line: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        agg_tokens.extend(split_line
            .iter()
            .map(|s| Token::Text{t:s.to_string()}));
        agg_tokens.push(Token::EOL);
    };
    return agg_tokens;
}

fn parse_opcode(input: String) -> Option<OpCode> {
    return match input.as_str() {
        "DAT" => Option::from(OpCode::DAT),
        "MOV" => Option::from(OpCode::MOV),
        "ADD" => Option::from(OpCode::ADD),
        "SUB" => Option::from(OpCode::SUB),
        "MUL" => Option::from(OpCode::MUL),
        "DIV" => Option::from(OpCode::MUL),
        "MOD" => Option::from(OpCode::MOD),
        "JMP" => Option::from(OpCode::JMP),
        "JMZ" => Option::from(OpCode::JMZ),
        "JMN" => Option::from(OpCode::JMN),
        "DJN" => Option::from(OpCode::DJN),
        "CMP" => Option::from(OpCode::CMP),
        "SLT" => Option::from(OpCode::SLT),
        "SPL" => Option::from(OpCode::SPL),
        _     => Option::None
    }
}
