use thiserror::Error;

use crate::{code::Opcode, evaluator::object::Object};

#[derive(Debug, Error)]
pub enum MonkeyError<'a> {
    #[error("Opcode not found: {:?}", .0)]
    OpcodeNotFound(&'a Opcode),
    #[error("Unknown integer operator: {:?}", .0)]
    UnknownOperator(&'a Opcode),
    #[error("Max stack size reached")]
    StackOverflow,
    #[error("Empty stack")]
    EmptyStackException,
    #[error("Unsupported type for negation: {}", .0)]
    UnsupportedType(Object),
}

pub type Result<'a, T> = std::result::Result<T, MonkeyError<'a>>;
