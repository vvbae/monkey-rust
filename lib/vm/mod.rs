use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    code::{read_u16, read_u8, Opcode},
    common::oth,
    compiler::Bytecode,
    error::MonkeyError,
    error::Result,
    evaluator::{builtins::BuiltinsFunctions, object::Object},
};

use self::frame::Frame;

const STACK_SIZE: usize = 2048;
const GLOBAL_SIZE: usize = 65536;
const MAX_FRAMES: usize = 1024;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

pub struct VM {
    constants: Vec<Object>,

    stack: RefCell<Vec<Object>>,
    sp: RefCell<usize>,

    globals: RefCell<Vec<Object>>,

    frames: RefCell<Vec<Frame>>,
    frame_index: RefCell<usize>,

    curr_frame: RefCell<Frame>, // a workaround for immutable borrow
    curr_frame_index: RefCell<usize>,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        let main_fn = Object::CompiledFn(bytecode.instructions.clone(), 0, 0);
        let main_closure = Object::Closure(Rc::new(main_fn), Vec::new());
        let main_frame = Frame::new(main_closure, 0);

        let mut frames = Vec::with_capacity(MAX_FRAMES);
        frames.push(main_frame.clone());

        Self {
            constants: bytecode.constants,
            stack: RefCell::new(vec![Object::Null; STACK_SIZE]),
            sp: RefCell::new(0),
            globals: RefCell::new(vec![Object::Null; GLOBAL_SIZE]),
            frames: RefCell::new(frames),
            frame_index: RefCell::new(1),
            curr_frame: RefCell::new(main_frame),
            curr_frame_index: RefCell::new(0),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut current_frame = self.curr_frame.borrow_mut().clone();

        while current_frame.ip < current_frame.instructions().len() as i64 - 1 {
            current_frame.ip += 1;

            let ip = current_frame.ip as usize;
            let ins = current_frame.instructions();
            let op = Opcode::from(&ins[ip as usize]);
            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&ins[ip + 1..ip + 3]);
                    current_frame.ip += 2;
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
                    current_frame.ip += 2; // continue to consequence

                    let condition = self.pop()?;
                    if !Self::is_truthy(condition) {
                        current_frame.ip = pos as i64 - 1; // jump to alternative
                    }
                }
                Opcode::OpJump => {
                    let pos = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    current_frame.ip = pos as i64 - 1;
                }
                Opcode::OpNull => self.push(NULL)?,
                Opcode::OpGetGlobal => {
                    let globals = self.globals.borrow();
                    let global_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    current_frame.ip += 2;

                    self.push(globals[global_index].clone())?;
                }
                Opcode::OpSetGlobal => {
                    let mut globals = self.globals.borrow_mut();
                    let global_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    current_frame.ip += 2;

                    let value = self.pop()?;
                    globals[global_index] = value;
                }
                Opcode::OpArray => {
                    let array = {
                        let mut sp = self.sp.borrow_mut();
                        let num_ele = read_u16(&ins[ip + 1..ip + 3]) as usize;
                        current_frame.ip += 2;

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
                        current_frame.ip += 2;

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
                Opcode::OpCall => {
                    let num_args = read_u8(&ins[ip + 1]) as usize;
                    current_frame.ip += 1;

                    current_frame = self.execute_call(num_args, current_frame.clone());
                }
                Opcode::OpReturnValue | Opcode::OpReturn => {
                    let return_val = match op {
                        Opcode::OpReturnValue => self.pop()?,
                        Opcode::OpReturn => Object::Null,
                        _ => unimplemented!(),
                    };

                    // decrement frame index
                    let mut frame_index = self.frame_index.borrow_mut();
                    *frame_index -= 1;

                    // sync current frame to the frames vec
                    let mut frames = self.frames.borrow_mut();
                    let mut curr_frame_index = self.curr_frame_index.borrow_mut();
                    frames[*curr_frame_index] = current_frame.clone();

                    // reset stack pointer after return
                    {
                        let mut sp = self.sp.borrow_mut();
                        *sp = current_frame.base_pointer - 1;
                    }

                    // update current frame
                    current_frame = frames[*curr_frame_index - 1].clone();
                    *curr_frame_index -= 1;

                    self.push(return_val)?;
                }
                Opcode::OpGetLocal => {
                    let local_index = read_u8(&ins[ip + 1]);
                    current_frame.ip += 1;

                    let local_val = {
                        let stack = self.stack.borrow();
                        stack[current_frame.base_pointer + local_index as usize].clone()
                    };
                    self.push(local_val)?;
                }
                Opcode::OpSetLocal => {
                    let local_index = read_u8(&ins[ip + 1]);
                    current_frame.ip += 1;

                    let value = self.pop()?;
                    let mut stack = self.stack.borrow_mut();
                    // take the value off stack and insert in reserved slot

                    let index = current_frame.base_pointer + local_index as usize;
                    stack[index] = value;
                }
                Opcode::OpGetBuiltin => {
                    let builtin_index = read_u8(&ins[ip + 1]) as usize;
                    current_frame.ip += 1;

                    let (_, builtin_fn) = &BuiltinsFunctions.get_builtins()[builtin_index];
                    self.push(builtin_fn.clone())?;
                }
                Opcode::OpClosure => {
                    let fn_index = read_u16(&ins[ip + 1..ip + 3]) as usize;
                    let num_free = read_u8(&ins[ip + 3]);
                    current_frame.ip += 3;

                    self.push_closure(fn_index, num_free as usize)?;
                }
                Opcode::OpGetFree => {
                    let free_index = read_u8(&ins[ip + 1]) as usize;
                    current_frame.ip += 1;

                    let free = {
                        if let Object::Closure(_, free) = current_frame.cl.clone() {
                            free.clone()
                        } else {
                            vec![]
                        }
                    };

                    self.push(free[free_index].clone())?;
                }
            }
        }

        Ok(())
    }

    fn push_closure(&self, fn_index: usize, num_free: usize) -> Result<()> {
        let closure = {
            let func = self.constants[fn_index].clone();
            let stack = self.stack.borrow();
            let mut sp = self.sp.borrow_mut();

            let mut frees = vec![Object::Null; num_free];
            frees.clone_from_slice(&stack[*sp - num_free..*sp]);
            *sp -= num_free;
            Object::Closure(Rc::new(func), frees)
        };
        self.push(closure)?;

        Ok(())
    }

    fn execute_call(&self, num_args: usize, curr_frame: Frame) -> Frame {
        let (frame, obj) = {
            let stack = self.stack.borrow();
            let mut sp = self.sp.borrow_mut();
            let callee = stack[*sp - 1 - num_args].clone();

            match callee.clone() {
                Object::Closure(_compiled_fn, _free_vars) => {
                    let frame = Frame::new(callee.clone(), *sp - num_args);
                    let base_pointer = frame.base_pointer;

                    // push frame
                    let mut frames = self.frames.borrow_mut();
                    let mut frame_index = self.frame_index.borrow_mut();

                    if *frame_index >= frames.len() {
                        frames.push(frame);
                    } else {
                        frames[*frame_index] = frame;
                    }
                    *frame_index += 1;

                    // starting point is base_pointer, and reserve for locals
                    if let Object::Closure(func, _) = callee {
                        if let Object::CompiledFn(_, num_locals, _) = Rc::as_ref(&func) {
                            *sp = base_pointer + *num_locals as usize;
                        }
                    }

                    // update current frame, and sync it to the frames vec
                    let mut curr_frame_index = self.curr_frame_index.borrow_mut();
                    frames[*curr_frame_index] = curr_frame;

                    *curr_frame_index += 1;
                    (frames[*curr_frame_index].clone(), None)
                }
                Object::Builtin(name, _arg, func) => {
                    let args = {
                        let mut vacant = vec![Object::Null; num_args];
                        vacant.clone_from_slice(&stack[*sp - num_args..*sp]);
                        vacant
                    };

                    let result = func(args);
                    *sp = *sp - 1 - num_args;

                    let obj = match result.clone() {
                        Ok(builtin_ret) => builtin_ret,
                        Err(_) => Object::Error(format!("invalid arguments for {}", name)),
                    };

                    (curr_frame, Some(obj))
                }
                _ => unimplemented!(),
            }
        };

        match obj {
            None => frame,
            Some(ret) => {
                let _ = self.push(ret);
                frame
            }
        }
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
            (Object::Hash(map), _) => self.execute_hash_index(map.clone(), index),
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

        stack[*sp] = obj;
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

pub mod frame;
#[cfg(test)]
mod test;
