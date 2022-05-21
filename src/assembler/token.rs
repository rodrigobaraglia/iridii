use crate::instructions::Opcode;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Operator(Opcode),
    Register(u8),
    Integer(i32),
    EOF,
}

impl From<&str> for Token {
    fn from(v: &str) -> Self {
        Token::Operator(Opcode::from(v))
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
       
        match v {
            operator::HLT => Opcode::HLT,
            operator::LOAD => Opcode::LOAD,
            operator::ADD => Opcode::ADD,
            operator::SUB => Opcode::SUB,
            operator::MUL => Opcode::MUL,
            operator::DIV => Opcode::DIV,
            operator::JMP => Opcode::JMP,
            operator::JMPF => Opcode::JMPF,
            operator::JMPB => Opcode::JMPB,
            operator::EQ => Opcode::EQ,
            operator::NEQ => Opcode::NEQ,
            operator::GT => Opcode::GT,
            operator::LT => Opcode::GT,
            operator::GTEQ => Opcode::GT,
            operator::LTEQ => Opcode::LTEQ,
            operator::JEQ => Opcode::LTEQ,
            _ => Opcode::ILGL,
        }
    }
}

mod operator {
    pub const HLT: &'static str = "hlt";
    pub const LOAD: &'static str = "load";
    pub const ADD: &'static str = "add";
    pub const SUB: &'static str = "sub";
    pub const MUL: &'static str = "mul";
    pub const DIV: &'static str = "div";
    pub const JMP: &'static str = "jmp";
    pub const JMPF: &'static str = "jmpf";
    pub const JMPB: &'static str = "jmpb";
    pub const EQ: &'static str = "eq";
    pub const NEQ: &'static str = "neq";
    pub const GT: &'static str = "gt";
    pub const LT: &'static str = "lt";
    pub const GTEQ: &'static str = "gteq";
    pub const LTEQ: &'static str = "lteq";
    pub const JEQ: &'static str = "jeq";
    pub const ILGL: &'static str = "ilgl";
}

mod prefix {
    pub const REGISTER: char = '$';
    pub const VALUE: char = '#';
}

fn is_letter(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_digit(ch: char) -> bool {
    matches!(ch, '0'..='9')
}

fn is_whitespace(ch: char) -> bool {
    matches!(ch, ' ' | '\t' | '\n' | '\r')
}

struct Cursor<'a>(Chars<'a>);

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    cursor: Chars<'a>,
    pos: usize,
    next_pos: usize,
    ch: char,
    col: usize,
    ln: usize,
}

// TODO: abstract lexer into a library
impl Lexer<'_> {
    pub fn new(input: &String) -> Lexer {
        let mut l = Lexer {
            input,
            cursor: input.chars(),
            pos: 0,
            next_pos: 0,
            ln: 0,
            col: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.ch == '\0' {
            self.col = 0;
            self.ln += 1;
        }
        self.ch = self.cursor.next().unwrap_or('\0');
        self.pos = self.next_pos;
        self.next_pos += 1;
        self.col += 1;
    }

    fn read_while(&mut self, predicate: impl Fn(char) -> bool) -> (usize, usize, usize, usize) {
        let start = self.pos;
        let col = self.col;
        let ln = self.ln;
        let mut cursor = self.cursor.clone();
        while predicate(cursor.next().unwrap_or('\0')) {
            self.read_char()
        }
        (start, self.next_pos, ln, col)
    }

    fn read_identifier(&mut self) -> Token {
        let (start, end, _ln, _col) = self.read_while(&is_letter);
        let span = &self.input[start..end];
        Token::from(span)
    }

    fn read_integer(&mut self) -> Token {
        let (start, end, _ln, _col) = self.read_while(&is_digit);
        let literal = &self.input[start..end];
        let num: i32 = literal.parse().unwrap_or(0);
        Token::Integer(num)
    }

    fn read_register(&mut self) -> Token {
        let (start, end, _ln, _col) = self.read_while(&is_digit);
        let literal = &self.input[start..end];
        let num: u8 = literal.parse().unwrap_or(0);
        Token::Register(num)
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            prefix::REGISTER => self.read_register(),
            prefix::VALUE => self.read_integer(),
            ch if is_letter(ch) => self.read_identifier(),
            '\0' => Token::EOF,
            _ => Token::Operator(Opcode::ILGL),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::EOF => None,
            token => Some(token),
        }
    }
}
