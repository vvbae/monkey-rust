use crate::{code::Instructions, evaluator::object::Object};

#[derive(Debug, Clone)]
pub struct Frame {
    pub func: Object,
    pub ip: i64,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(func: Object, base_pointer: usize) -> Self {
        Self {
            func,
            ip: -1,
            base_pointer,
        }
    }

    pub fn instructions(&self) -> Instructions {
        if let Object::CompiledFn(ins, _, _) = &self.func {
            return ins.to_vec();
        }

        vec![]
    }
}
