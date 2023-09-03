use thiserror::Error;

use crate::code::Opcode;

#[derive(Debug, Error)]
pub enum MonkeyError<'a> {
    #[error("Opcode not found")]
    OpcodeNotFound(&'a Opcode),
}

pub type Result<'a, T> = std::result::Result<T, MonkeyError<'a>>;
