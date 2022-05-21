#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTEQ,
    LTEQ,
    JEQ,
    ALLOC,
    ILGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}
// ¿se puede reemplazar esto representando el programa como un vector de opcodes en lugar de un vector de bytes?
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTEQ,
            14 => Opcode::LTEQ,
            15 => Opcode::JEQ,
            16 => Opcode::ALLOC,
            _ => Opcode::ILGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_hlt() {
        let op = Opcode::HLT;
        assert_eq!(op, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let i = Instruction::new(Opcode::HLT);
        assert_eq!(i.opcode, Opcode::HLT);
    }
}
