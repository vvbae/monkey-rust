use std::{cell::RefCell, collections::HashMap};

use crate::{
    code::{read_u16, Instructions, Opcode},
    common::oth,
    compiler::Bytecode,
    error::MonkeyError,
    error::Result,
    evaluator::object::Object,
};

const STACK_SIZE: usize = 2048;
const GLOBAL_SIZE: usize = 65536;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

pub struct VM {
    constants: Vec<Object>,
    instructions: RefCell<Instructions>,
    stack: RefCell<Vec<Object>>,
    sp: RefCell<usize>,
    globals: RefCell<Vec<Object>>,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            constants: bytecode.constants,
            instructions: RefCell::new(bytecode.instructions),
            stack: RefCell::new(Vec::with_capacity(STACK_SIZE)),
            sp: RefCell::new(0),
            globals: RefCell::new(Vec::with_capacity(GLOBAL_SIZE)),
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
                Opcode::OpTrue => self.push(TRUE)?,
                Opcode::OpFalse => self.push(FALSE)?,
                Opcode::OpEqual | Opcode::OpNotEqual | Opcode::OpGreaterThan => {
                    self.execute_comparison(op)?
                }
                Opcode::OpMinus => self.execute_minus_operator()?,
                Opcode::OpBang => self.execute_bang_operator()?,
                Opcode::OpJumpNotTruthy => {
                    let pos = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2; // continue to consequence

                    let condition = self.pop()?;
                    if !Self::is_truthy(condition) {
                        ip = pos - 1; // jump to alternative
                    }
                }
                Opcode::OpJump => {
                    let pos = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip = pos - 1;
                }
                Opcode::OpNull => self.push(NULL)?,
                Opcode::OpGetGlobal => {
                    let globals = self.globals.borrow();
                    let global_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;

                    self.push(globals[global_index].clone())?;
                }
                Opcode::OpSetGlobal => {
                    let mut globals = self.globals.borrow_mut();
                    let global_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;

                    let value = self.pop()?;
                    if global_index >= globals.len() {
                        globals.push(value);
                    } else {
                        globals[global_index] = value;
                    }
                }
                Opcode::OpArray => {
                    let array = {
                        let mut sp = self.sp.borrow_mut();
                        let num_ele = read_u16(&ins[ip + 1..ip + 3]) as usize;
                        ip += 2;

                        let array = self.build_array(*sp - num_ele, *sp);
                        *sp -= num_ele;

                        array
                    };

                    self.push(array)?;
                }
                Opcode::OpHash => {
                    let hashmap = {
                        let mut sp = self.sp.borrow_mut();
                        let num_ele = read_u16(&ins[ip + 1..ip + 3]) as usize;
                        ip += 2;

                        let hash = self.build_hash(*sp - num_ele, *sp);
                        *sp -= num_ele;

                        hash
                    };

                    self.push(hashmap)?;
                }
                Opcode::OpIndex => {
                    let index = self.pop()?;
                    let left = self.pop()?;

                    self.execute_index_expr(&left, &index)?;
                }
            }

            ip += 1
        }

        Ok(())
    }

    fn build_array(&self, start_index: usize, end_index: usize) -> Object {
        let stack = self.stack.borrow();
        let mut eles = Vec::with_capacity(end_index - start_index);
        eles.extend_from_slice(&stack[start_index..end_index]);

        Object::Array(eles)
    }

    fn build_hash(&self, start_index: usize, end_index: usize) -> Object {
        let mut hashed_pairs = HashMap::new();
        let stack = self.stack.borrow();

        for i in (start_index..end_index).step_by(2) {
            let key = stack[i].clone();
            let val = stack[i + 1].clone();

            let hash_key = oth(key.clone());
            hashed_pairs.insert(hash_key, val);
        }

        Object::Hash(hashed_pairs)
    }

    fn execute_binary_operation(&self, op: Opcode) -> Result<()> {
        let right = self.pop()?;
        let left = self.pop()?;

        let res = match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => self.execute_binary_int_operation(op, l, r),
            (Object::String(l), Object::String(r)) => {
                self.execute_binary_string_operation(op, l, r)
            }
            _ => unimplemented!(),
        }?;

        self.push(res)?;

        Ok(())
    }

    fn execute_binary_int_operation(
        &self,
        op: Opcode,
        left_val: i64,
        right_val: i64,
    ) -> Result<Object> {
        let res = match op {
            Opcode::OpAdd => right_val + left_val,
            Opcode::OpSub => left_val - right_val,
            Opcode::OpMul => right_val * left_val,
            Opcode::OpDiv => left_val / right_val,
            _ => unimplemented!("Unknown integer operator found: {:?}", op),
        };

        Ok(Object::Integer(res))
    }

    fn execute_binary_string_operation(
        &self,
        op: Opcode,
        left_val: String,
        right_val: String,
    ) -> Result<Object> {
        if op != Opcode::OpAdd {
            return Err(MonkeyError::UnknownOperator);
        }

        Ok(Object::String(left_val + &right_val))
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

    fn execute_bang_operator(&self) -> Result<()> {
        let operand = self.pop()?;

        match operand {
            Object::Boolean(v) => match v {
                true => self.push(FALSE),
                false => self.push(TRUE),
            },
            Object::Null => self.push(TRUE),
            _ => self.push(FALSE),
        }?;

        Ok(())
    }

    fn execute_minus_operator(&self) -> Result<()> {
        let operand = self.pop()?;

        match operand {
            Object::Integer(v) => self.push(Object::Integer(-v)),
            _ => Err(MonkeyError::UnsupportedType(operand)),
        }?;

        Ok(())
    }

    fn execute_index_expr(&self, left: &Object, index: &Object) -> Result<()> {
        match (left, index) {
            (Object::Array(array), Object::Integer(id)) => {
                self.execute_array_index(array.to_vec(), *id)
            }
            (Object::Hash(map), Object::Integer(_) | Object::String(_)) => {
                self.execute_hash_index(map.clone(), index)
            }
            _ => unimplemented!("index operator not supported: {:?}", left),
        }?;

        Ok(())
    }

    fn execute_array_index(&self, array: Vec<Object>, index: i64) -> Result<()> {
        let max = array.len() as i64 - 1;

        if index < 0 || index > max as i64 {
            return self.push(NULL);
        }

        self.push(array[index as usize].clone())?;

        Ok(())
    }

    fn execute_hash_index(&self, map: HashMap<Object, Object>, index: &Object) -> Result<()> {
        let key = oth(index.clone());
        let val = map.get(&key).unwrap_or(&Object::Null);

        self.push(val.clone())?;

        Ok(())
    }

    fn is_truthy(obj: Object) -> bool {
        match obj {
            Object::Boolean(v) => v,
            Object::Null => false,
            _ => true,
        }
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
