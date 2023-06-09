use crate::{stack::Stack, Instruction};
use num_traits::FromPrimitive;
use runtime::{Closure, NIL};
use std::mem::size_of;
use std::str;

macro_rules! binary_operation {
    ($self:expr, $operator:tt) => {
        let value = (|| {
            let lhs = $self.stack.pop_value().to_float64()?;
            let rhs = $self.stack.pop_value().to_float64()?;

            Some((lhs.to_f64() $operator rhs.to_f64()).into())
        })()
        .unwrap_or(NIL);

        $self.stack.push_value(value);
    };
}

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
        while self.program_counter < instructions.len() {
            let instruction = Instruction::from_u8(instructions[self.program_counter])
                .expect("valid instruction");

            self.program_counter += 1;

            match instruction {
                Instruction::Null => unreachable!("null po' god!"),
                Instruction::Nil => {
                    self.stack.push_value(NIL);
                }
                Instruction::Float64 => {
                    let value = self.read_u64(instructions);
                    self.stack.push_value(f64::from_bits(value).into());
                }
                Instruction::Symbol => {
                    let len = self.read_u8(instructions);
                    let value = str::from_utf8(self.read_bytes(instructions, len as usize))
                        .unwrap()
                        .into();

                    self.stack.push_value(value);
                }
                Instruction::Get => {
                    let value = (|| {
                        let array = self.stack.pop_value().into_array()?;

                        Some(array.get(self.stack.pop_value()))
                    })()
                    .unwrap_or(NIL);

                    self.stack.push_value(value);
                }
                Instruction::Set => {
                    let value = (|| {
                        let array = self.stack.pop_value().into_array()?;
                        let index = self.stack.pop_value();
                        let value = self.stack.pop_value();

                        Some(array.set(index, value).into())
                    })()
                    .unwrap_or(NIL);

                    self.stack.push_value(value);
                }
                Instruction::Length => {
                    let value = (|| Some(self.stack.pop_value().into_array()?.len().into()))()
                        .unwrap_or(NIL);

                    self.stack.push_value(value);
                }
                Instruction::Add => {
                    binary_operation!(self, +);
                }
                Instruction::Subtract => {
                    binary_operation!(self, -);
                }
                Instruction::Multiply => {
                    binary_operation!(self, *);
                }
                Instruction::Divide => {
                    binary_operation!(self, /);
                }
                Instruction::Drop => {
                    self.stack.pop_value();
                }
                Instruction::Dump => {
                    let value = self.stack.pop_value();

                    println!("{}", value);

                    self.stack.push_value(value);
                }
                Instruction::Call => todo!(),
                Instruction::Closure => {
                    let id = self.read_u32(instructions);
                    let environment_size = self.read_u8(instructions) as usize;
                    let mut closure = Closure::new(id, environment_size);

                    for index in 0..environment_size {
                        let variable_index = self.read_u8(instructions);

                        closure.write_environment(
                            index,
                            self.stack.get(variable_index as usize).clone(),
                        );
                    }

                    self.stack.push_value(closure.into());
                }
                Instruction::Local => {
                    // TODO Check a frame pointer.
                    // TODO Move local variables when possible.
                    let index = self.read_u8(instructions);
                    self.stack
                        .push_value(self.stack.get(index as usize).clone());
                }
                Instruction::Equal => todo!(),
                Instruction::Array => todo!(),
                Instruction::Jump => self.program_counter = self.read_u32(instructions) as usize,
                Instruction::Return => todo!(),
            }
        }
    }

    fn read_u64(&mut self, instructions: &[u8]) -> u64 {
        const SIZE: usize = size_of::<u64>();
        let mut bytes = [0u8; SIZE];

        bytes.copy_from_slice(&instructions[self.program_counter..self.program_counter + SIZE]);

        self.program_counter += SIZE;

        u64::from_le_bytes(bytes)
    }

    fn read_u32(&mut self, instructions: &[u8]) -> u32 {
        const SIZE: usize = size_of::<u32>();
        let mut bytes = [0u8; SIZE];

        bytes.copy_from_slice(&instructions[self.program_counter..self.program_counter + SIZE]);

        self.program_counter += SIZE;

        u32::from_le_bytes(bytes)
    }

    fn read_u8(&mut self, instructions: &[u8]) -> u8 {
        let value = instructions[self.program_counter];

        self.program_counter += 1;

        value
    }

    fn read_bytes<'a>(&mut self, instructions: &'a [u8], len: usize) -> &'a [u8] {
        let value = &instructions[self.program_counter..self.program_counter + len];

        self.program_counter += len;

        value
    }
}
