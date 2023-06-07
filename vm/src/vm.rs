use crate::{stack::Stack, Instruction};
use std::mem::transmute;

const STACK_SIZE: usize = 2048;

pub struct Vm {
    program_counter: usize,
    stack: Stack,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn run(&mut self, instructions: &[u8]) {
        for &instruction in instructions {
            match unsafe { transmute(instruction) } {
                Instruction::Nil => panic!("nil po' god!"),
                Instruction::Float64 => todo!(),
                Instruction::Float64Add => {
                    self.stack
                        .push_f64(self.stack.pop_f64() + self.stack.pop_f64());
                }
                Instruction::Float64Subtract => {
                    self.stack
                        .push_f64(self.stack.pop_f64() - self.stack.pop_f64());
                }
                Instruction::Float64Multiply => {
                    self.stack
                        .push_f64(self.stack.pop_f64() * self.stack.pop_f64());
                }
                Instruction::Float64Divide => {
                    self.stack
                        .push_f64(self.stack.pop_f64() / self.stack.pop_f64());
                }
                Instruction::Call => todo!(),
                Instruction::Lambda => todo!(),
            }
        }
    }
}
