use crate::{
    code::{make, Instructions, Opcode},
    evaluator::object::Object,
    parser::ast::{Expr, Ident, Infix, Literal, Prefix, Program, Stmt},
};

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Instructions::new(),
            constants: Vec::new(),
        }
    }

    pub fn compile(&mut self, program: Program) {
        for stmt in program.iter() {
            self.compile_statement(stmt.clone());
        }
    }

    pub fn compile_statement(&mut self, stmt: Stmt) -> Object {
        match stmt {
            Stmt::ExprStmt(expr) => self.compile_expr(expr),
            Stmt::LetStmt(ident, expr) => {
                // some thing with ident
                self.compile_expr(expr)
            }
            Stmt::ReturnStmt(expr) => self.compile_expr(expr),
        }
    }

    pub fn compile_expr(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::IdentExpr(i) => self.compile_ident(i),
            Expr::LitExpr(l) => {
                let lit = self.compile_literal(l);
                let const_index = self.register_constant(&lit) as u16;
                self.emit(Opcode::OpConstant, vec![const_index]);
                lit
            }
            Expr::PrefixExpr(prefix, expr) => self.compile_prefix(&prefix, *expr),
            Expr::InfixExpr(infix, expr1, expr2) => self.compile_infix(&infix, *expr1, *expr2),
            Expr::IfExpr {
                cond,
                consequence,
                alternative,
            } => self.compile_if(*cond, consequence, alternative),
            Expr::FnExpr { params, body } => self.compile_fn(params, body),
            Expr::CallExpr {
                function: fn_exp,
                arguments,
            } => self.compile_call(*fn_exp, arguments),
            Expr::ArrayExpr(exprs) => self.compile_array(exprs),
            Expr::HashExpr(hash_exprs) => self.compile_hash(hash_exprs),
            Expr::IndexExpr { array, index } => self.compile_index(*array, *index),
        }
    }

    pub fn compile_ident(&self, ident: Ident) -> Object {
        todo!()
    }

    pub fn compile_literal(&self, lit: Literal) -> Object {
        match lit {
            Literal::IntLiteral(v) => Object::Integer(v),
            Literal::BoolLiteral(v) => Object::Boolean(v),
            Literal::StringLiteral(v) => Object::String(v),
        }
    }

    pub fn compile_prefix(&self, pre: &Prefix, expr: Expr) -> Object {
        todo!()
    }

    pub fn compile_infix(&mut self, infix: &Infix, expr1: Expr, expr2: Expr) -> Object {
        let obj1 = self.compile_expr(expr1);
        let obj2 = self.compile_expr(expr2);
        obj1
    }

    pub fn compile_if(
        &self,
        cond: Expr,
        consequence: Vec<Stmt>,
        alternative: Option<Vec<Stmt>>,
    ) -> Object {
        todo!()
    }

    pub fn compile_fn(&self, params: Vec<Ident>, body: Vec<Stmt>) -> Object {
        todo!()
    }

    pub fn compile_call(&self, fn_exp: Expr, arg: Vec<Expr>) -> Object {
        todo!()
    }

    pub fn compile_array(&self, exprs: Vec<Expr>) -> Object {
        todo!()
    }

    pub fn compile_hash(&self, hash_exprs: Vec<(Literal, Expr)>) -> Object {
        todo!()
    }

    pub fn compile_index(&self, array: Expr, index: Expr) -> Object {
        todo!()
    }

    /// Append obj to constants, return its index as identifier for the OpConstant instruction
    fn register_constant(&mut self, obj: &Object) -> usize {
        self.constants.push(obj.clone());
        self.constants.len() - 1
    }

    /// Generate an instruction and return its position
    fn emit(&mut self, op: Opcode, operands: Vec<u16>) -> usize {
        let ins = make(op, operands);
        let pos = self.add_instruction(ins);
        pos
    }

    /// Add the new instruction to memory, return its position
    fn add_instruction(&mut self, mut ins: Vec<u8>) -> usize {
        let pos_new_ins = self.instructions.len();
        self.instructions.append(&mut ins);
        pos_new_ins
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
}

pub struct Bytecode {
    instructions: Instructions,
    constants: Vec<Object>,
}

#[cfg(test)]
mod test;
