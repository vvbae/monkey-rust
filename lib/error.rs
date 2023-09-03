use thiserror::Error;

use crate::code::Opcode;

#[derive(Debug, Error)]
pub enum MonkeyError<'a> {
    #[error("Opcode not found")]
    OpcodeNotFound(&'a Opcode),
    #[error("Unknown integer operator")]
    UnknownOperator(&'a Opcode),
    #[error("Max stack size reached")]
    StackOverflow,
    #[error("Empty stack")]
    EmptyStackException,
}

pub type Result<'a, T> = std::result::Result<T, MonkeyError<'a>>;