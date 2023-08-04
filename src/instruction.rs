#[derive(PartialEq)]
#[derive(Debug)]
pub enum Opcode {
    HLT,
    IGL
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
