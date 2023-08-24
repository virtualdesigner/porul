use std::{io, io::Write, num::ParseIntError};
use crate::vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new()
        }
    }

    pub fn run(&mut self) -> () {
        println!("Welcome! Write your Kurals!");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read user input");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            
            match buffer {
                ".quit" => {
                    println!("See you!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{command}");
                    }
                }
                ".registers" => {
                    println!("{:?}", self.vm.registers);
                }
                _ => {
                    match self.parse_hex(buffer) {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        },
                        Err(err) => panic!("Failed to parse the hex input: {buffer}. Error: {err}")
                    };
                    self.vm.run_once();
                }
            }
        }
    }

    pub fn parse_hex(&self, input: &str) -> Result<Vec<u8>, ParseIntError> {
        let splitted_input = input.split(" ").collect::<Vec<&str>>();
        let mut parsed_instructions = vec![];
        for instruction in splitted_input {
            let integer = u8::from_str_radix(instruction, 16);
            match integer {
                Ok(parsed_int) => parsed_instructions.push(parsed_int),
                Err(err) => return Err(err),
            }
        }
        Ok(parsed_instructions)
    }
}