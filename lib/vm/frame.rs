use crate::{code::Instructions, evaluator::object::Object};

#[derive(Debug, Clone)]
pub struct Frame {
    pub func: Object,
    pub ip: i64,
}

impl Frame {
    pub fn new(func: Object) -> Self {
        Self { func, ip: -1 }
    }

    pub fn instructions(&self) -> Instructions {
        if let Object::CompiledFn(ins, _, _) = &self.func {
            return ins.to_vec();
        }

        vec![]
    }
}
