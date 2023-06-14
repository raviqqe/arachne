use crate::{
    decode::{decode_bytes, decode_f64, decode_u16, decode_u32, decode_u8},
    frame::Frame,
    stack::Stack,
    Instruction,
};
use num_traits::FromPrimitive;
use runtime::{Closure, NIL};
use std::str;

macro_rules! binary_operation {
    ($self:expr, $operator:tt) => {
        let value = (|| {
            let rhs = $self.stack.pop().to_float64()?;
            let lhs = $self.stack.pop().to_float64()?;

            Some((lhs.to_f64() $operator rhs.to_f64()).into())
        })()
        .unwrap_or(NIL);

        $self.stack.push(value);
    };
}

pub struct Vm {
    program_counter: usize,
    stack: Stack,
    frames: Vec<Frame>,
}

impl Vm {
    pub fn new(stack_size: usize) -> Self {
        Self {
            program_counter: 0,
            stack: Stack::new(stack_size),
            frames: Default::default(),
        }
    }

    pub fn run(&mut self, codes: &[u8]) {
        while self.program_counter < codes.len() {
            match Instruction::from_u8(self.read_u8(codes)).expect("valid instruction") {
                Instruction::Nil => self.stack.push(NIL),
                Instruction::Float64 => {
                    let value = self.read_f64(codes);
                    self.stack.push(value.into());
                }
                Instruction::Integer32 => {
                    let value = self.read_u32(codes);
                    self.stack.push(value.into());
                }
                Instruction::Symbol => {
                    let len = self.read_u8(codes);
                    let value = str::from_utf8(self.read_bytes(codes, len as usize))
                        .unwrap()
                        .into();

                    self.stack.push(value);
                }
                Instruction::Get => {
                    let value = (|| {
                        let index = self.stack.pop();
                        let array = self.stack.pop().into_array()?;

                        Some(array.get(index))
                    })()
                    .unwrap_or(NIL);

                    self.stack.push(value);
                }
                Instruction::Set => {
                    let value = (|| {
                        let value = self.stack.pop();
                        let index = self.stack.pop();
                        let array = self.stack.pop().into_array()?;

                        Some(array.set(index, value).into())
                    })()
                    .unwrap_or(NIL);

                    self.stack.push(value);
                }
                Instruction::Length => {
                    let value =
                        (|| Some(self.stack.pop().into_array()?.len().into()))().unwrap_or(NIL);

                    self.stack.push(value);
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
                    self.stack.pop();
                }
                Instruction::Dump => {
                    let value = self.stack.pop();

                    println!("{}", value);

                    self.stack.push(value);
                }
                Instruction::Call => {
                    let arity = self.read_u8(codes) as usize;

                    if let Some(closure) = self.stack.peek(arity).as_closure() {
                        let id = closure.id();
                        let closure_arity = closure.arity() as usize;

                        self.frames.push(Frame::new(
                            self.program_counter,
                            self.stack.len() - arity - 1,
                        ));
                        self.program_counter = id as usize;

                        for _ in 0..arity.saturating_sub(closure_arity) {
                            self.stack.pop();
                        }

                        for _ in 0..closure_arity.saturating_sub(arity) {
                            self.stack.push(NIL);
                        }
                    } else {
                        for _ in 0..arity + 1 {
                            self.stack.pop();
                        }

                        self.stack.push(NIL);
                    }
                }
                Instruction::Close => {
                    let id = self.read_u32(codes);
                    let arity = self.read_u8(codes);
                    let environment_size = self.read_u8(codes);
                    let mut closure = Closure::new(id, arity, environment_size);

                    for index in 0..environment_size {
                        let variable_index = self.read_u8(codes);

                        closure.write_environment(
                            index as usize,
                            self.stack.peek(variable_index as usize).clone(),
                        );
                    }

                    self.stack.push(closure.into());
                }
                Instruction::Peek => {
                    // TODO Move local variables when possible.
                    let index = self.read_u8(codes);

                    self.stack.push(self.stack.peek(index as usize).clone());
                }
                Instruction::Equal => {
                    let rhs = self.stack.pop();
                    let lhs = self.stack.pop();

                    self.stack.push(((lhs == rhs) as usize as f64).into());
                }
                Instruction::Jump => {
                    let address = self.read_u16(codes);

                    self.program_counter = self
                        .program_counter
                        .wrapping_add(address as i16 as isize as usize);
                }
                Instruction::Branch => {
                    let address = self.read_u16(codes);
                    let value = self.stack.pop();

                    if value != NIL {
                        self.program_counter = self
                            .program_counter
                            .wrapping_add(address as i16 as isize as usize);
                    }
                }
                Instruction::Return => {
                    let value = self.stack.pop();
                    let frame = self.frames.pop().expect("frame");

                    // TODO Remove an operand.
                    self.read_u8(codes);

                    while self.stack.len() > frame.frame_pointer() {
                        self.stack.pop();
                    }

                    self.program_counter = frame.return_address();

                    self.stack.push(value);
                }
            }
        }
    }

    fn read_f64(&mut self, codes: &[u8]) -> f64 {
        decode_f64(codes, &mut self.program_counter)
    }

    fn read_u32(&mut self, codes: &[u8]) -> u32 {
        decode_u32(codes, &mut self.program_counter)
    }

    fn read_u16(&mut self, codes: &[u8]) -> u16 {
        decode_u16(codes, &mut self.program_counter)
    }

    fn read_u8(&mut self, codes: &[u8]) -> u8 {
        decode_u8(codes, &mut self.program_counter)
    }

    fn read_bytes<'a>(&mut self, codes: &'a [u8], len: usize) -> &'a [u8] {
        decode_bytes(codes, len, &mut self.program_counter)
    }
}
