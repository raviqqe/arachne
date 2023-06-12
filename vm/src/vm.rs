use crate::{
    decode::{decode_bytes, decode_f64, decode_u32, decode_u8},
    stack::Stack,
    Instruction,
};
use num_traits::FromPrimitive;
use runtime::{Closure, NIL};
use std::str;

macro_rules! binary_operation {
    ($self:expr, $operator:tt) => {
        let value = (|| {
            let rhs = $self.stack.pop_value().to_float64()?;
            let lhs = $self.stack.pop_value().to_float64()?;

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

    pub fn run(&mut self, codes: &[u8]) {
        while self.program_counter < codes.len() {
            match Instruction::from_u8(self.read_u8(codes)).expect("valid instruction") {
                Instruction::Nil => self.stack.push_value(NIL),
                Instruction::Float64 => {
                    let value = self.read_f64(codes);
                    self.stack.push_value(value.into());
                }
                Instruction::Symbol => {
                    let len = self.read_u8(codes);
                    let value = str::from_utf8(self.read_bytes(codes, len as usize))
                        .unwrap()
                        .into();

                    self.stack.push_value(value);
                }
                Instruction::Get => {
                    let value = (|| {
                        let index = self.stack.pop_value();
                        let array = self.stack.pop_value().into_array()?;

                        Some(array.get(index))
                    })()
                    .unwrap_or(NIL);

                    self.stack.push_value(value);
                }
                Instruction::Set => {
                    let value = (|| {
                        let value = self.stack.pop_value();
                        let index = self.stack.pop_value();
                        let array = self.stack.pop_value().into_array()?;

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
                    let id = self.read_u32(codes);
                    let environment_size = self.read_u8(codes) as usize;
                    let mut closure = Closure::new(id, environment_size);

                    for index in 0..environment_size {
                        let variable_index = self.read_u8(codes);

                        closure.write_environment(
                            index,
                            self.stack.get(variable_index as usize).clone(),
                        );
                    }

                    self.stack.push_value(closure.into());
                }
                Instruction::Local => {
                    // TODO Move local variables when possible.
                    let index = self.read_u8(codes);

                    self.stack
                        .push_value(self.stack.get(index as usize).clone());
                }
                Instruction::Equal => todo!(),
                Instruction::Array => todo!(),
                // TODO Make this relative jump.
                Instruction::Jump => self.program_counter = self.read_u32(codes) as usize,
                Instruction::Return => todo!(),
            }
        }
    }

    fn read_f64(&mut self, codes: &[u8]) -> f64 {
        decode_f64(codes, &mut self.program_counter)
    }

    fn read_u32(&mut self, codes: &[u8]) -> u32 {
        decode_u32(codes, &mut self.program_counter)
    }

    fn read_u8(&mut self, codes: &[u8]) -> u8 {
        decode_u8(codes, &mut self.program_counter)
    }

    fn read_bytes<'a>(&mut self, codes: &'a [u8], len: usize) -> &'a [u8] {
        decode_bytes(codes, len, &mut self.program_counter)
    }
}
