#[derive(PartialEq)]
#[derive(Debug)]
pub enum Opcode {
    HLT,
    IGL,
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
    JEQ,
    GEQ,
    LEQ,
    GT,
    LT,
    JNEQ
}

pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::JEQ,
            12 => return Opcode::GEQ,
            13 => return Opcode::LEQ,
            14 => return Opcode::GT,
            15 => return Opcode::LT,
            16 => return Opcode::JNEQ,
            _ => return Opcode::IGL
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hld() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
