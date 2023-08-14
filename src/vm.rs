use crate::instruction::Opcode;
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
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
}