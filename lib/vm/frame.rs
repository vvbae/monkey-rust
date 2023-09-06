use std::rc::Rc;

use crate::{code::Instructions, evaluator::object::Object};

#[derive(Debug, Clone)]
pub struct Frame {
    pub cl: Object,
    pub ip: i64,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(cl: Object, base_pointer: usize) -> Self {
        Self {
            cl,
            ip: -1,
            base_pointer,
        }
    }

    pub fn instructions(&self) -> Instructions {
        if let Object::Closure(func, _) = &self.cl {
            if let Object::CompiledFn(ins, _, _) = Rc::as_ref(func) {
                return ins.to_vec();
            }
        }

        vec![]
    }
}
