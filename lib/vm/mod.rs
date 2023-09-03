use std::cell::RefCell;

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
    instructions: RefCell<Instructions>,

    stack: RefCell<Vec<Object>>,
    sp: RefCell<usize>,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            constants: bytecode.constants,
            instructions: RefCell::new(bytecode.instructions),
            stack: RefCell::new(Vec::with_capacity(STACK_SIZE)),
            sp: RefCell::new(0),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let ins = self.instructions.borrow();
        let mut ip = 0;

        while ip < ins.len() {
            let op = Opcode::from(&ins[ip]);

            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&ins[ip + 1..ip + 3]);
                    ip += 2;
                    self.push(self.constants[const_index as usize].clone())?;
                }
                Opcode::OpAdd | Opcode::OpDiv | Opcode::OpSub | Opcode::OpMul => {
                    self.execute_binary_operation(op)?;
                }
                Opcode::OpPop => {
                    self.pop()?;
                }
            }

            ip += 1
        }

        Ok(())
    }

    fn execute_binary_operation(&self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;

        let left_val = match left {
            Object::Integer(v) => v,
            _ => todo!(),
        };

        let right_val = match right {
            Object::Integer(v) => v,
            _ => todo!(),
        };

        let res = match op {
            Opcode::OpAdd => right_val + left_val,
            Opcode::OpSub => left_val - right_val,
            Opcode::OpMul => right_val * left_val,
            Opcode::OpDiv => left_val / right_val,
            _ => unimplemented!("Unknown integer operator found: {:?}", op),
        };

        self.push(Object::Integer(res))?;

        Ok(())
    }

    /// Push obj to the top of the stack
    pub fn push(&self, obj: Object) -> Result<()> {
        let mut sp = self.sp.borrow_mut();
        let mut stack = self.stack.borrow_mut();

        if *sp >= STACK_SIZE {
            return Err(MonkeyError::StackOverflow);
        }

        // FIXME: maybe change declaration to fill capacity
        if *sp >= stack.len() {
            stack.push(obj);
        } else {
            stack[*sp] = obj;
        }

        *sp += 1;

        Ok(())
    }

    /// Decrement stack pointer, return last obj
    pub fn pop(&self) -> Result<Object> {
        let mut sp = self.sp.borrow_mut();
        let stack = self.stack.borrow();

        if stack.len() == 0 {
            return Err(MonkeyError::EmptyStackException);
        }

        let obj = &stack[*sp - 1];
        *sp -= 1;

        Ok(obj.clone())
    }

    /// Last ele previously on the stack
    pub fn last_popped_stack_ele(&self) -> Object {
        let sp = self.sp.borrow();
        let stack = self.stack.borrow();

        stack[*sp].clone()
    }

    pub fn stack_top(&self) -> Option<Object> {
        let sp = self.sp.borrow();
        let stack = self.stack.borrow();

        if *sp == 0 {
            return None;
        }

        Some(stack[*sp - 1].clone())
    }
}

#[cfg(test)]
mod test;
