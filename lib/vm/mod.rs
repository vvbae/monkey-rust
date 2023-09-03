use std::cell::RefCell;

use crate::{
    code::{read_u16, Instructions, Opcode},
    compiler::Bytecode,
    error::MonkeyError,
    error::Result,
    evaluator::object::Object,
};

const STACK_SIZE: usize = 2048;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);

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
                Opcode::OpTrue => {
                    self.push(TRUE)?;
                }
                Opcode::OpFalse => {
                    self.push(FALSE)?;
                }
                Opcode::OpEqual | Opcode::OpNotEqual | Opcode::OpGreaterThan => {
                    self.execute_comparison(op)?;
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

    fn execute_comparison(&self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;

        match (&left, &right) {
            (Object::Integer(_l), Object::Integer(_r)) => {
                return self.execute_int_comparison(op, &left, &right);
            }
            _ => {
                match op {
                    Opcode::OpEqual => self.push(native_to_object(right == left)),
                    Opcode::OpNotEqual => self.push(native_to_object(right != left)),
                    _ => {
                        unimplemented!("Unknown operator found: {:?} ({:?} {:?})", op, left, right)
                    }
                }?;
            }
        }

        Ok(())
    }

    fn execute_int_comparison(&self, op: Opcode, left: &Object, right: &Object) -> Result<()> {
        let left_val = match left {
            Object::Integer(v) => v,
            _ => todo!(),
        };

        let right_val = match right {
            Object::Integer(v) => v,
            _ => todo!(),
        };

        match op {
            Opcode::OpEqual => self.push(native_to_object(right_val == left_val)),
            Opcode::OpNotEqual => self.push(native_to_object(right_val != left_val)),
            Opcode::OpGreaterThan => self.push(native_to_object(left_val > right_val)),
            _ => unimplemented!("Unknown operator found: {:?} ({:?} {:?})", op, left, right),
        }?;

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

pub(super) fn native_to_object(input: bool) -> Object {
    match input {
        true => TRUE,
        false => FALSE,
    }
}

#[cfg(test)]
mod test;
