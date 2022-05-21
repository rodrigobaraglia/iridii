use crate::assembler::token::{Lexer, Token};
use crate::instructions::Opcode;

mod err {
    pub const integer_for_register: &'static str = "syntax error: expected register, found integer";
    pub const operator_for_register: &'static str =
        "syntax error: expected register, found operator";
    pub const register_for_integer: &'static str = "syntax error: expected integer, found register";
    pub const operator_for_integer: &'static str = "syntax error: expected integer, found operator";
}

fn parse_integer(i: i32) -> (u8, u8) {
    let right = i as u16;
    let left = right >> 8;
    (left as u8, right as u8)
}

struct Parser<'a> {
    lexer: Lexer<'a>,
} // is this wrapper **really**¨ necessary?

impl<'a> Iterator for Parser<'a> {
    type Item = [u8; 4];
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_instruction() {
            Ok(instruction) => Some(instruction),
            Err(e) => panic!("{e}"),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a String) -> Self {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    fn register(&mut self) -> Result<u8, &'static str> {
        match self.lexer.next_token() {
            Token::Register(address) => Ok(address),
            Token::Integer(_) => Err(err::integer_for_register),
            Token::Operator(_) => Err(err::operator_for_register),
            _ => Err("EOF"),
        }
    }

    fn integer(&mut self) -> Result<(u8, u8), &'static str> {
        match self.lexer.next_token() {
            Token::Integer(value) => Ok(parse_integer(value)),
            Token::Register(_) => Err(err::register_for_integer),
            Token::Operator(_) => Err(err::operator_for_integer),
            _ => Err("EOF"),
        }
    }

    fn integer_op(&mut self, op: Opcode) -> Result<[u8; 4], &'static str> {
        let op = op as u8;
        let output_address = self.register()?;
        let (left_byte, right_byte) = self.integer()?;
        Ok([op, output_address, left_byte, right_byte])
    }

    fn nullary_op(&mut self, op: Opcode) -> Result<[u8; 4], &'static str> {
        let op = op as u8;
        Ok([op, 0, 0, 0])
    }

    fn binary_op(&mut self, op: Opcode) -> Result<[u8; 4], &'static str> {
        let op = op as u8;
        let left_operand = self.register()?;
        let right_operand = self.register()?;
        let output_address = self.register()?;
        Ok([op, left_operand, right_operand, output_address])
    }

    fn next_instruction(&mut self) -> Result<[u8; 4], &'static str> {
        let token = self.lexer.next_token();

        match token {
            Token::Operator(Opcode::LOAD) => self.integer_op(Opcode::LOAD),
            Token::Operator(Opcode::ADD) => self.binary_op(Opcode::ADD),
            Token::Operator(Opcode::SUB) => self.binary_op(Opcode::SUB),
            _ => Err("EOF"),
        }
    }
}
