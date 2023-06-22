mod ir;

use super::super::{
    decode::{decode_bytes, decode_u16, decode_u32, decode_u64, decode_u8},
    Instruction,
};
use core::fmt;
use ir::InstructionIr;
use num_traits::FromPrimitive;
use std::{
    self,
    error::Error,
    fmt::{Display, Formatter},
    str::{self, Utf8Error},
};

pub fn format_instructions(codes: &[u8]) -> Result<String, FormatError> {
    let mut index = 0;
    let mut instructions = Vec::new();

    while index < codes.len() {
        let instruction = decode_u8(codes, &mut index);

        instructions.push(
            match Instruction::from_u8(instruction)
                .ok_or(FormatError::InvalidInstruction(instruction))?
            {
                Instruction::Nil => InstructionIr::Nil,
                Instruction::Float64 => {
                    InstructionIr::Float64(f64::from_bits(decode_u64(codes, &mut index)))
                }
                Instruction::Integer32 => {
                    InstructionIr::Integer32(decode_u32(codes, &mut index) as i32)
                }
                Instruction::Symbol => {
                    let len = decode_u8(codes, &mut index);

                    InstructionIr::Symbol {
                        len,
                        string: str::from_utf8(decode_bytes(codes, len as usize, &mut index))?
                            .into(),
                    }
                }
                Instruction::Peek => InstructionIr::Peek(decode_u8(codes, &mut index)),
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
                Instruction::TailCall => InstructionIr::TailCall {
                    arity: decode_u8(codes, &mut index),
                },
                Instruction::Close => InstructionIr::Close {
                    pointer: decode_u32(codes, &mut index),
                    arity: decode_u8(codes, &mut index),
                    environment_size: decode_u8(codes, &mut index),
                },
                Instruction::Environment => {
                    InstructionIr::Environment(decode_u8(codes, &mut index))
                }
                Instruction::Equal => InstructionIr::Equal,
                Instruction::LessThan => InstructionIr::LessThan,
                Instruction::Not => InstructionIr::Not,
                Instruction::And => InstructionIr::And,
                Instruction::Or => InstructionIr::Or,
                Instruction::Drop => InstructionIr::Drop,
                Instruction::Dump => InstructionIr::Dump,
                Instruction::Jump => InstructionIr::Jump {
                    pointer: decode_u16(codes, &mut index) as i16,
                },
                Instruction::Branch => InstructionIr::Branch {
                    pointer: decode_u16(codes, &mut index) as i16,
                },
                Instruction::Return => InstructionIr::Return,
            },
        );
    }

    Ok(instructions
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n"))
}

#[derive(Debug)]
pub enum FormatError {
    InvalidInstruction(u8),
    Utf8(Utf8Error),
}

impl Error for FormatError {}

impl Display for FormatError {
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

impl From<Utf8Error> for FormatError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}
