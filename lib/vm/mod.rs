use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    error::MonkeyError,
    error::Result,
    evaluator::object::Object,
};

const STACK_SIZE: usize = 2048;

pub struct VM {
    constants: Vec<Object>,
    instructions: Instructions,

    stack: Vec<Object>,
    sp: usize,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::with_capacity(STACK_SIZE),
            sp: 0,
        }
    }

    pub fn stack_top(&self) -> Option<&Object> {
        if self.sp == 0 {
            return None;
        }

        Some(&self.stack[self.sp - 1])
    }

    pub fn run(&mut self) -> Result<()> {
        let ins = self.instructions.clone();
        let mut ip = 0;

        while ip < ins.len() {
            let op = Opcode::from(&ins[ip]);

            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&self.instructions[ip + 1..ip + 3]);
                    ip += 2;

                    // FIXME: current is a workaround for not borrowing as immutable and mutable
                    self.sp = Self::push(
                        &mut self.stack,
                        self.sp,
                        self.constants[const_index as usize].clone(),
                    )?;
                }
                Opcode::OpAdd => {
                    let (right, sp) = Self::pop(&mut self.stack, self.sp)?;
                    self.sp = sp;
                    let (left, sp) = Self::pop(&mut self.stack, self.sp)?;
                    self.sp = sp;

                    let left_val = match left {
                        Object::Integer(v) => v,
                        _ => todo!(),
                    };

                    let right_val = match right {
                        Object::Integer(v) => v,
                        _ => todo!(),
                    };

                    let res = right_val + left_val;
                    self.sp = Self::push(&mut self.stack, self.sp, Object::Integer(res))?;
                }
            }

            ip += 1
        }

        Ok(())
    }

    /// Push obj to the top of the stack
    pub fn push(stack: &mut Vec<Object>, sp: usize, obj: Object) -> Result<'static, usize> {
        if sp >= STACK_SIZE {
            return Err(MonkeyError::StackOverflow);
        }

        stack.push(obj);
        Ok(sp + 1)
    }

    /// Pop obj from the stack
    pub fn pop(stack: &mut Vec<Object>, sp: usize) -> Result<'static, (Object, usize)> {
        if stack.len() == 0 {
            return Err(MonkeyError::EmptyStackException);
        }

        Ok((stack.pop().unwrap(), sp - 1))
    }
}

#[cfg(test)]
mod test;
