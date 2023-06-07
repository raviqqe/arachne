use crate::{stack::Stack, Instruction};
use runtime::{Value, NIL};
use std::mem::{size_of, transmute};

macro_rules! binary_operation {
    ($self:expr, $operator:tt) => {
        let value = (|| {
            let lhs = $self.stack.pop_value().to_float64()?;
            let rhs = $self.stack.pop_value().to_float64()?;

            Some((lhs.to_f64() $operator rhs.to_f64()).into())
        })()
        .unwrap_or(NIL);

        $self.stack.push_value(value);
        $self.program_counter += 1;
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
        self.program_counter = 0;

        while self.program_counter < instructions.len() {
            match unsafe { transmute(instructions[self.program_counter]) } {
                Instruction::Nil => unreachable!("nil po' god!"),
                Instruction::Constant => {
                    let value = self.read_value(instructions);
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
                Instruction::Float64Add => {
                    binary_operation!(self, +);
                }
                Instruction::Float64Subtract => {
                    binary_operation!(self, -);
                }
                Instruction::Float64Multiply => {
                    binary_operation!(self, *);
                }
                Instruction::Float64Divide => {
                    binary_operation!(self, /);
                }
                Instruction::Call => todo!(),
                Instruction::Lambda => todo!(),
            }
        }
    }

    fn read_value(&mut self, instructions: &[u8]) -> Value {
        const SIZE: usize = size_of::<Value>();
        let mut bytes = [0u8; SIZE];

        bytes.copy_from_slice(&instructions[self.program_counter..self.program_counter + SIZE]);

        self.program_counter += SIZE;

        unsafe { Value::from_raw(u64::from_le_bytes(bytes)) }
    }
}
