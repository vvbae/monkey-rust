use byteorder::{BigEndian, ByteOrder};

pub type Instructions = Vec<u8>;

pub fn make(op: Opcode, operands: Option<Vec<u16>>) -> Instructions {
    let widths = op.look_up();
    let operands = operands.unwrap_or(vec![]);
    let instruction_len: usize = 1 + widths.iter().sum::<u8>() as usize;

    let mut instructions = vec![0; instruction_len];
    instructions[0] = Opcode::to_byte(op);

    let mut offset = 1;

    for (i, o) in operands.iter().enumerate() {
        let width = widths[i];
        match width {
            2 => instructions[offset..offset + 2].copy_from_slice(&o.to_be_bytes()),
            1 => instructions[offset] = (o & 0xFF) as u8, // truncate the upper 8 bits
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
        let widths = op.look_up();

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
        2 => format!("{} {} {}", opcode_str, operands[0], operands[1]),
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
            1 => operands[i] = read_u8(&ins[offset]) as u16,
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

/// Read u8 from instruction
pub fn read_u8(ins: &u8) -> u8 {
    *ins
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    OpConstant,
    OpAdd,
    OpPop,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
    OpEqual,
    OpNotEqual,
    OpGreaterThan,
    OpMinus,
    OpBang,
    OpJumpNotTruthy,
    OpJump,
    OpNull,
    OpGetGlobal,
    OpSetGlobal,
    OpArray,
    OpHash,
    OpIndex,
    OpCall,
    OpReturnValue,
    OpReturn,
    OpGetLocal,
    OpSetLocal,
    OpGetBuiltin,
    OpClosure,
    OpGetFree,
}

impl Opcode {
    /// Look up the operand width for given opcode
    fn look_up(&self) -> Vec<u8> {
        match self {
            Opcode::OpConstant => vec![2],
            Opcode::OpAdd => vec![],
            Opcode::OpPop => vec![],
            Opcode::OpSub => vec![],
            Opcode::OpMul => vec![],
            Opcode::OpDiv => vec![],
            Opcode::OpTrue => vec![],
            Opcode::OpFalse => vec![],
            Opcode::OpEqual => vec![],
            Opcode::OpNotEqual => vec![],
            Opcode::OpGreaterThan => vec![],
            Opcode::OpMinus => vec![],
            Opcode::OpBang => vec![],
            Opcode::OpJumpNotTruthy => vec![2],
            Opcode::OpJump => vec![2],
            Opcode::OpNull => vec![],
            Opcode::OpGetGlobal => vec![2],
            Opcode::OpSetGlobal => vec![2],
            Opcode::OpArray => vec![2],
            Opcode::OpHash => vec![2],
            Opcode::OpIndex => vec![],
            Opcode::OpCall => vec![1],
            Opcode::OpReturnValue => vec![],
            Opcode::OpReturn => vec![],
            Opcode::OpGetLocal => vec![1],
            Opcode::OpSetLocal => vec![1],
            Opcode::OpGetBuiltin => vec![1],
            Opcode::OpClosure => vec![2, 1],
            Opcode::OpGetFree => vec![1],
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
            Opcode::OpTrue => 6,
            Opcode::OpFalse => 7,
            Opcode::OpEqual => 8,
            Opcode::OpNotEqual => 9,
            Opcode::OpGreaterThan => 10,
            Opcode::OpMinus => 11,
            Opcode::OpBang => 12,
            Opcode::OpJumpNotTruthy => 13,
            Opcode::OpJump => 14,
            Opcode::OpNull => 15,
            Opcode::OpGetGlobal => 16,
            Opcode::OpSetGlobal => 17,
            Opcode::OpArray => 18,
            Opcode::OpHash => 19,
            Opcode::OpIndex => 20,
            Opcode::OpCall => 21,
            Opcode::OpReturnValue => 22,
            Opcode::OpReturn => 23,
            Opcode::OpGetLocal => 24,
            Opcode::OpSetLocal => 25,
            Opcode::OpGetBuiltin => 26,
            Opcode::OpClosure => 27,
            Opcode::OpGetFree => 28,
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
            6 => Opcode::OpTrue,
            7 => Opcode::OpFalse,
            8 => Opcode::OpEqual,
            9 => Opcode::OpNotEqual,
            10 => Opcode::OpGreaterThan,
            11 => Opcode::OpMinus,
            12 => Opcode::OpBang,
            13 => Opcode::OpJumpNotTruthy,
            14 => Opcode::OpJump,
            15 => Opcode::OpNull,
            16 => Opcode::OpGetGlobal,
            17 => Opcode::OpSetGlobal,
            18 => Opcode::OpArray,
            19 => Opcode::OpHash,
            20 => Opcode::OpIndex,
            21 => Opcode::OpCall,
            22 => Opcode::OpReturnValue,
            23 => Opcode::OpReturn,
            24 => Opcode::OpGetLocal,
            25 => Opcode::OpSetLocal,
            26 => Opcode::OpGetBuiltin,
            27 => Opcode::OpClosure,
            28 => Opcode::OpGetFree,
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
            Opcode::OpTrue => "OpTrue",
            Opcode::OpFalse => "OpFalse",
            Opcode::OpEqual => "OpEqual",
            Opcode::OpNotEqual => "OpNotEqual",
            Opcode::OpGreaterThan => "OpGreaterThan",
            Opcode::OpMinus => "OpMinus",
            Opcode::OpBang => "OpBang",
            Opcode::OpJumpNotTruthy => "OpJumpNotTruthy",
            Opcode::OpJump => "OpJump",
            Opcode::OpNull => "OpNull",
            Opcode::OpGetGlobal => "OpGetGlobal",
            Opcode::OpSetGlobal => "OpSetGlobal",
            Opcode::OpArray => "OpArray",
            Opcode::OpHash => "OpHash",
            Opcode::OpIndex => "OpIndex",
            Opcode::OpCall => "OpCall",
            Opcode::OpReturnValue => "OpReturnValue",
            Opcode::OpReturn => "OpReturn",
            Opcode::OpGetLocal => "OpGetLocal",
            Opcode::OpSetLocal => "OpSetLocal",
            Opcode::OpGetBuiltin => "OpGetBuiltin",
            Opcode::OpClosure => "OpClosure",
            Opcode::OpGetFree => "OpGetFree",
        }
        .to_string()
    }
}

#[cfg(test)]
mod test;
