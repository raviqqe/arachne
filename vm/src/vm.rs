use crate::{stack::Stack, Instruction};
use runtime::Value;
use std::mem::{size_of, transmute};

pub struct Vm {
    program_counter: usize,
    stack: Stack,
}

impl Vm {
    pub fn new(stack_size: usize) -> Self {
        Self {
            program_counter: 0,
            stack: Stack::new(stack_size),
        }
    }

    pub fn run(&mut self, instructions: &[u8]) {
        self.program_counter = 0;

        while self.program_counter < instructions.len() {
            match unsafe { transmute(instructions[self.program_counter]) } {
                Instruction::Nil => panic!("nil po' god!"),
                Instruction::Constant => {
                    let value = self.read_value(instructions);
                    self.stack.push_value(value);
                }
                Instruction::Float64Add => {
                    let lhs = self.stack.pop_f64();
                    let rhs = self.stack.pop_f64();

                    self.stack.push_f64(lhs + rhs);
                    self.program_counter += 1;
                }
                Instruction::Float64Subtract => {
                    let lhs = self.stack.pop_f64();
                    let rhs = self.stack.pop_f64();

                    self.stack.push_f64(lhs + rhs);

                    self.program_counter += 1;
                }
                Instruction::Float64Multiply => {
                    let lhs = self.stack.pop_f64();
                    let rhs = self.stack.pop_f64();

                    self.stack.push_f64(lhs + rhs);
                }
                Instruction::Float64Divide => {
                    let lhs = self.stack.pop_f64();
                    let rhs = self.stack.pop_f64();

                    self.stack.push_f64(lhs + rhs);
                }
                Instruction::Call => todo!(),
                Instruction::Lambda => todo!(),
            }
        }
    }

    fn read_value(&mut self, instructions: &[u8]) -> Value {
        let size = size_of::<Value>();
        let mut bytes = [0u8; 8];

        bytes.copy_from_slice(&instructions[self.program_counter..self.program_counter + size]);

        self.program_counter += size;

        return unsafe { Value::from_raw(u64::from_le_bytes(bytes)) };
    }
}
