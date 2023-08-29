use std::process;

use crate::assembler::{Token, opcode_parsers::opcode_load, operand_parsers::integer_operand, register_parsers::register};
use nom::{named, do_parse, types::CompleteStr, many1};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

named!(
    pub load_instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_load >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op{code} => match code {
                _ => {
                    results.push(code as u8);
                    println!("Opcode found");
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(token) => match token {
                    Token::Register { reg_number } => {
                        results.push(*reg_number);
                    },
                    Token::IntegerOperand { value } => {
                        let converted = *value as u16;
                        let value1 = converted as u8;
                        let value2 = (converted >> 8) as u8;
                        results.push(value2);
                        results.push(value1);
                    },
                    _ => {
                        println!("Wrong code type found in operand field");
                        std::process::exit(1);
                    }
                },
                None => ()
            }
        }
        results
    }
}

pub struct Program {
    instructions: Vec<AssemblerInstruction>
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];

        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }

        program
    }
}

named!(
    pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(load_instruction) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use crate::instruction::Opcode;

    use super::*;


    #[test]
    fn test_parse_load_instruction() {
        let result = load_instruction(CompleteStr("load $1 #10"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(token, 
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::LOAD },
                operand1: Some(Token::Register { reg_number: 1 }),
                operand2: Some(Token::IntegerOperand { value: 10 }),
                operand3: None
            }
        );

        let result = load_instruction(CompleteStr("Load 1 10"));
        assert_eq!(result.is_ok(), false);
    }
    
    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $1 #100"));
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:#?}", bytecode);
    }
}