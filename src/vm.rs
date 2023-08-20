use crate::instruction::Opcode;
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    comparison_result: bool
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            comparison_result: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT Encountered!");
                true
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits();
                self.registers[register] = number as i32;
                false
            }
            Opcode::ADD => {
                let number_1 = self.registers[self.next_8_bits() as usize];
                let number_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = number_1 + number_2;
                false
            }
            Opcode::SUB => {
                let number_1 = self.registers[self.next_8_bits() as usize];
                let number_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = number_1 - number_2;
                false
            }
            Opcode::MUL => {
                let number_1 = self.registers[self.next_8_bits() as usize];
                let number_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = number_1 * number_2;
                false
            }
            Opcode::DIV => {
                let number_1 = self.registers[self.next_8_bits() as usize];
                let number_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = number_1 / number_2;
                self.remainder = (number_1 % number_2) as u32;
                false
            }
            Opcode::JMP => {
                // absolute jump
                let step_to_jump = ((self.next_16_bits() as u32) << 8 | self.next_8_bits() as u32) as usize;
                if step_to_jump >= self.program.len() {
                    panic!("The instruction index: {step_to_jump} to jump is larger than the program length: {}", self.program.len());
                }
                self.pc = step_to_jump;
                false
            }
            Opcode::JMPF => {
                // relative jump forward
                let step_to_jump = ((self.next_16_bits() as u32) << 8 | (self.next_8_bits()) as u32) as usize;
                if self.pc + step_to_jump >= self.program.len() {
                    panic!("The instruction index: {} to jump forward is larger than the program length: {}", self.pc + step_to_jump, self.program.len());
                }
                self.pc += step_to_jump;
                false
            }
            Opcode::JMPB => {
                // relative jump backward
                let step_to_jump = ((self.next_16_bits() as u32) << 8 | (self.next_8_bits()) as u32) as usize;
                if (self.pc as i32 - step_to_jump as i32) < 0 {
                    panic!("The instruction index: {} to jump backward is negative: {}", self.pc - step_to_jump, self.program.len());
                }
                self.pc -= step_to_jump;
                false
            }
            Opcode::EQ => {
                let value_1 = self.registers[self.next_8_bits() as usize];
                let value_2 = self.registers[self.next_8_bits() as usize];
                self.next_8_bits();

                if value_1 == value_2 {
                    self.comparison_result = true;
                } else {
                    self.comparison_result = false;
                }
                false
            }
            Opcode::NEQ => {
                let value_1 = self.registers[self.next_8_bits() as usize];
                let value_2 = self.registers[self.next_8_bits() as usize];
                self.next_8_bits();

                if value_1 != value_2 {
                    self.comparison_result = true;
                } else {
                    self.comparison_result = false;
                }
                false
            }
            Opcode::JEQ => {
                let register_index = self.next_8_bits() as usize;
                self.next_16_bits();

                if register_index >= 32 {
                    panic!("JEQ command referring to non-existing register index: {register_index}")
                }
                let step_to_jump = self.registers[register_index];
                if self.comparison_result == true {
                    if step_to_jump as usize > self.program.len() {
                        panic!("The instruction index: {} to jump is greater than the program length: {}", step_to_jump, self.program.len());
                    } else if step_to_jump < 0 {
                        panic!("The instruction index: {} to jump is negative", step_to_jump);
                    }
                    self.pc = step_to_jump as usize;
                }
                false
            }
            Opcode::GEQ => {
                let value_1 = self.registers[self.next_8_bits() as usize];
                let value_2 = self.registers[self.next_8_bits() as usize];
                self.next_8_bits();

                if value_1 >= value_2 {
                    self.comparison_result = true;
                } else {
                    self.comparison_result = false;
                }
                false
            }
            Opcode::LEQ => {
                let value_1 = self.registers[self.next_8_bits() as usize];
                let value_2 = self.registers[self.next_8_bits() as usize];
                self.next_8_bits();

                if value_1 <= value_2 {
                    self.comparison_result = true;
                } else {
                    self.comparison_result = false;
                }
                false
            }
            Opcode::GT => {
                let value_1 = self.registers[self.next_8_bits() as usize];
                let value_2 = self.registers[self.next_8_bits() as usize];
                self.next_8_bits();

                if value_1 > value_2 {
                    self.comparison_result = true;
                } else {
                    self.comparison_result = false;
                }
                false
            }
            Opcode::LT => {
                let value_1 = self.registers[self.next_8_bits() as usize];
                let value_2 = self.registers[self.next_8_bits() as usize];
                self.next_8_bits();

                if value_1 < value_2 {
                    self.comparison_result = true;
                } else {
                    self.comparison_result = false;
                }
                false
            }
            other => {
                println!("Error: Unrecognized opcode {:?}! Terminating!", other);
                return true;
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let first_part = (self.program[self.pc] as u16) << 8;
        let second_part = self.program[self.pc + 1] as u16;
        self.pc += 2;
        return first_part | second_part;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_absolute_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![6, 0, 0, 1];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }


    #[test]
    fn test_jmp_relative_forward_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![7, 0, 0, 1, 1, 0, 0, 1];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_jmp_relative_backward_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![8, 0, 0, 2];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 20;
        test_vm.registers[2] = 20;
        test_vm.registers[3] = 21;

        test_vm.program = vec![
                            9, 1, 3, 0, 
                            9, 1, 2, 0
                        ];
        
        // check not equal to
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, false);

        // check equal to
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 20;
        test_vm.registers[2] = 20;
        test_vm.registers[3] = 21;

        test_vm.program = vec![
                            10, 1, 2, 0, 
                            10, 2, 3, 0
                        ];
        
        // check equal to
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, false);

        // check not equal to
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;

        test_vm.program = vec![
                            11, 0, 0, 0, 
                            11, 0, 0, 0, 
                            1, 1, 0, 10,
                        ];
        
        // check no jump when not equals
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);

        // check jump when equals
        test_vm.comparison_result = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 10);
    }

    #[test]
    fn test_geq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 22;
        test_vm.registers[2] = 20;
        test_vm.registers[3] = 21;
        test_vm.registers[4] = 21;

        test_vm.program = vec![
                            12, 2, 3, 0, 
                            12, 1, 2, 0,
                            12, 3, 4, 0,
                        ];
        
        // check not greater than or equal to
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, false);

        // check greater than
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);

        // check equals
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);
    }

    #[test]
    fn test_leq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 22;
        test_vm.registers[2] = 20;
        test_vm.registers[3] = 21;
        test_vm.registers[4] = 21;

        test_vm.program = vec![
                            13, 1, 2, 0, 
                            13, 2, 3, 0,
                            13, 3, 4, 0,
                        ];
        
        // check not less than or equal to
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, false);

        // check less than
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);

        // check equals
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 20;
        test_vm.registers[2] = 21;

        test_vm.program = vec![
                            14, 1, 2, 0, 
                            14, 2, 1, 0
                        ];
        
        // check not greater than
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, false);

        // check greater than
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 20;
        test_vm.registers[2] = 21;

        test_vm.program = vec![
                            15, 2, 1, 0, 
                            15, 1, 2, 0
                        ];
        
        // check not less than
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, false);

        // check less than
        test_vm.run_once();
        assert_eq!(test_vm.comparison_result, true);
    }
}