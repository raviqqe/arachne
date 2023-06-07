use crate::Instruction;
use std::mem::transmute;

const STACK_SIZE: usize = 2048;

pub struct Vm {
    program_counter: usize,
    stack: Vec<u64>,
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
                Instruction::Float64 => todo!(),
                Instruction::Add => todo!(),
                Instruction::Subtract => todo!(),
                Instruction::Multiply => todo!(),
                Instruction::Divide => todo!(),
                Instruction::Lambda => todo!(),
            }
        }
    }
}
