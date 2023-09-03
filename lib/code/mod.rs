use byteorder::{BigEndian, ByteOrder};

use crate::error::{MonkeyError, Result};

pub type Instructions = Vec<u8>;

// FIXME I dont know why result is not working here because of lifetime issue.
pub fn make(op: Opcode, operands: Option<Vec<u16>>) -> Instructions {
    let widths = op.look_up().unwrap();
    let operands = operands.unwrap_or(vec![]);
    let instruction_len: usize = 1 + widths.iter().sum::<u8>() as usize;

    let mut instructions = vec![0; instruction_len];
    instructions[0] = Opcode::to_byte(op);

    let mut offset = 1;

    for (i, o) in operands.iter().enumerate() {
        let width = widths[i];
        match width {
            2 => instructions[offset..offset + 2].copy_from_slice(&o.to_be_bytes()),
            _ => todo!(),
        }
        offset += width as usize;
    }

    instructions
}

pub fn string(ins: Instructions) -> String {
    let mut buffer = "".to_owned();
    let mut i = 0;

    while i < ins.len() {
        let op: Opcode = (&ins[i]).into();
        let widths = op.look_up().unwrap();

        let (operands, read) = read_operands(&widths, ins[(i + 1) as usize..].to_vec());
        let fmtted = fmt_ins(op, &widths, operands);

        buffer.push_str(&format!("{:04} {fmtted}\n", i));

        i += 1 + read as usize
    }

    buffer.to_string()
}

fn fmt_ins(op: Opcode, widths: &[u8], operands: Vec<u16>) -> String {
    let operand_cnt = widths.len();

    if operand_cnt != operands.len() {
        return format!(
            "ERROR: operand len {} does not match defined {}\n",
            operands.len(),
            operand_cnt
        );
    }

    let opcode_str: String = op.into();
    match operand_cnt {
        0 => format!("{}", opcode_str),
        1 => format!("{} {}", opcode_str, operands[0]),
        _ => format!("ERROR: unhandled operandCount for {}\n", opcode_str),
    }
}

/// Read operands from instruction given operands widths
pub fn read_operands(widths: &[u8], ins: Instructions) -> (Vec<u16>, u8) {
    let mut operands = vec![0; widths.len()];
    let mut offset = 0;

    for (i, width) in widths.iter().enumerate() {
        match width {
            2 => operands[i] = read_u16(&ins[offset..offset + 2]),
            _ => todo!(),
        }

        offset += *width as usize
    }

    (operands, offset as u8)
}

/// Read u16 from instruction
pub fn read_u16(ins: &[u8]) -> u16 {
    BigEndian::read_u16(&ins)
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    OpConstant,
    OpAdd,
    OpPop,
    OpSub,
    OpMul,
    OpDiv,
}

impl Opcode {
    /// Look up the operand width for given opcode
    fn look_up(&self) -> Result<Vec<u8>> {
        match self {
            Opcode::OpConstant => Ok(vec![2]),
            Opcode::OpAdd => Ok(vec![]),
            Opcode::OpPop => Ok(vec![]),
            Opcode::OpSub => Ok(vec![]),
            Opcode::OpMul => Ok(vec![]),
            Opcode::OpDiv => Ok(vec![]),
            _ => Err(MonkeyError::OpcodeNotFound(self)),
        }
    }

    /// Represent opcode as u8
    fn to_byte(op: Opcode) -> u8 {
        match op {
            Opcode::OpConstant => 0,
            Opcode::OpAdd => 1,
            Opcode::OpPop => 2,
            Opcode::OpSub => 3,
            Opcode::OpMul => 4,
            Opcode::OpDiv => 5,
        }
    }
}

impl From<&u8> for Opcode {
    fn from(v: &u8) -> Opcode {
        match v {
            0 => Opcode::OpConstant,
            1 => Opcode::OpAdd,
            2 => Opcode::OpPop,
            3 => Opcode::OpSub,
            4 => Opcode::OpMul,
            5 => Opcode::OpDiv,
            _ => todo!(),
        }
    }
}

impl Into<String> for Opcode {
    fn into(self) -> String {
        match self {
            Opcode::OpConstant => "OpConstant",
            Opcode::OpAdd => "OpAdd",
            Opcode::OpPop => "OpPop",
            Opcode::OpSub => "OpSub",
            Opcode::OpMul => "OpMul",
            Opcode::OpDiv => "OpDiv",
        }
        .to_string()
    }
}

#[cfg(test)]
mod test;
