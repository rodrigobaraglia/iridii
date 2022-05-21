use crate::assembler::token::{Lexer, Token};
use crate::instructions::Opcode;
use std::iter::{FromIterator, IntoIterator};
type Op = Opcode;
type Tok = Token;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    operator: Token,
    operands: Operands,
}

#[derive(Debug, PartialEq)]
struct Operands([Option<Token>; 3]);

impl FromIterator<Option<Token>> for Operands {
    fn from_iter<I: IntoIterator<Item = Option<Token>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut token = || iter.next().unwrap_or(None);
        Operands([token(), token(), token()])
    }
}


impl Lexer<'_> {
    fn read_instruction(&mut self) -> Instruction {
        let token = self.next_token();
        match token {
            Tok::Operator(Op::LOAD) => Instruction {
                operator: self.next_token(),
                operands: self.map(|operand| Some(operand)).collect::<Operands>(),
            },
            _ => panic!("unimplemented"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program(Vec<Instruction>);

fn parse_integer(i: i32) -> (u8, u8) {
    let right = i as u16;
    let left = right >> 8;
    (left as u8, right as u8)
}

impl Token {
    fn parse_operand(self, operands: &mut Vec<u8>) -> Result<(), &'static str> {
        match self {
            Token::Register(address) => operands.push(address),
            Token::Integer(value) => {
                let (left, right) = parse_integer(value);
                operands.push(left);
                operands.push(right);
            }
            _ => return Err("syntax error: operator in operand position"),
        }
        Ok(())
    }
}

impl Instruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>, &'static str> {
        if let Token::Operator(op) = self.operator {
            let mut bytes = vec![op as u8];
            let Operands(ops) = self.operands;
            for operand in ops.into_iter() {
                if let Some(token) = operand {
                    token.parse_operand(&mut bytes)?
                }
            }
            Ok(bytes)
        } else {
            Err("syntax error: opedand in operator position")
        }
    }
}
