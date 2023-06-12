use crate::{
    decode::{decode_bytes, decode_u16, decode_u32, decode_u64, decode_u8},
    Instruction,
};
use core::fmt;
use num_traits::FromPrimitive;
use std::{
    self,
    error::Error,
    fmt::{Display, Formatter},
    str::{self, Utf8Error},
};

#[derive(Clone, Debug)]
pub enum InstructionIr {
    Nil,
    Float64(f64),
    Symbol {
        len: u8,
        string: String,
    },
    Local(u8),
    Get,
    Set,
    Length,
    Add,
    Subtract,
    Multiply,
    Divide,
    Call {
        arity: u8,
    },
    Close {
        pointer: u32,
        arity: u8,
        environment_size: u8,
        environment: Vec<u8>,
    },
    Equal,
    Array,
    Drop,
    Dump,
    Jump {
        pointer: u16,
    },
    Return {
        frame_size: u8,
    },
}

pub fn decode_instructions(codes: &[u8]) -> Result<Vec<InstructionIr>, DecodeError> {
    let mut index = 0;
    let mut instructions = Vec::new();

    while index < codes.len() {
        let instruction = decode_u8(codes, &mut index);

        instructions.push(
            match Instruction::from_u8(instruction)
                .ok_or(DecodeError::InvalidInstruction(instruction))?
            {
                Instruction::Nil => InstructionIr::Nil,
                Instruction::Float64 => {
                    InstructionIr::Float64(f64::from_bits(decode_u64(codes, &mut index)))
                }
                Instruction::Symbol => {
                    let len = decode_u8(codes, &mut index);

                    InstructionIr::Symbol {
                        len,
                        string: str::from_utf8(decode_bytes(codes, len as usize, &mut index))?
                            .into(),
                    }
                }
                Instruction::Global => todo!(),
                Instruction::Local => InstructionIr::Local(decode_u8(codes, &mut index)),
                Instruction::Get => InstructionIr::Get,
                Instruction::Set => InstructionIr::Set,
                Instruction::Length => InstructionIr::Length,
                Instruction::Add => InstructionIr::Add,
                Instruction::Subtract => InstructionIr::Subtract,
                Instruction::Multiply => InstructionIr::Multiply,
                Instruction::Divide => InstructionIr::Divide,
                Instruction::Call => InstructionIr::Call {
                    arity: decode_u8(codes, &mut index),
                },
                Instruction::Close => {
                    let pointer = decode_u32(codes, &mut index);
                    let arity = decode_u8(codes, &mut index);
                    let environment_size = decode_u8(codes, &mut index);

                    InstructionIr::Close {
                        pointer,
                        arity,
                        environment_size,
                        environment: decode_bytes(codes, environment_size as usize, &mut index)
                            .to_vec(),
                    }
                }
                Instruction::Equal => InstructionIr::Equal,
                Instruction::Array => InstructionIr::Array,
                Instruction::Drop => InstructionIr::Drop,
                Instruction::Dump => InstructionIr::Dump,
                Instruction::Jump => InstructionIr::Jump {
                    pointer: decode_u16(codes, &mut index),
                },
                Instruction::Return => InstructionIr::Return {
                    frame_size: decode_u8(codes, &mut index),
                },
            },
        );
    }

    Ok(instructions)
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidInstruction(u8),
    Utf8(Utf8Error),
}

impl Error for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidInstruction(instruction) => {
                write!(formatter, "invalid instruction: {:x}", instruction)
            }
            Self::Utf8(error) => {
                write!(formatter, "{}", error)
            }
        }
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}
