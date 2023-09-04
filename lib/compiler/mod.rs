use crate::{
    code::{make, Instructions, Opcode},
    evaluator::object::Object,
    parser::ast::{Expr, Ident, Infix, Literal, Prefix, Program, Stmt},
};

use self::symbol_table::SymbolTable;

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,

    last_ins: Option<EmittedInstruction>,
    prev_ins: Option<EmittedInstruction>,

    symbol_table: SymbolTable,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Instructions::new(),
            constants: Vec::new(),
            last_ins: None,
            prev_ins: None,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn compile(&mut self, program: Program) {
        for stmt in program.iter() {
            self.compile_statement(stmt.clone());
        }
    }

    pub fn compile_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr);
                self.emit(Opcode::OpPop, None);
            }
            Stmt::LetStmt(ident, expr) => {
                self.compile_expr(expr);
                let symbol = self.symbol_table.define(ident.0);
                self.emit(Opcode::OpSetGlobal, Some(vec![symbol.index]));
            }
            Stmt::ReturnStmt(expr) => self.compile_expr(expr),
        };
    }

    pub fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::IdentExpr(i) => self.compile_ident(i),
            Expr::LitExpr(l) => self.compile_literal(l),
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
        };
    }

    pub fn compile_ident(&mut self, ident: Ident) {
        let symbol = self.symbol_table.resolve(ident.0).unwrap();
        self.emit(Opcode::OpGetGlobal, Some(vec![symbol.index]));
    }

    pub fn compile_literal(&mut self, lit: Literal) {
        match lit {
            Literal::IntLiteral(v) => {
                let lit = Object::Integer(v);
                let const_index = self.register_constant(&lit) as u16;
                self.emit(Opcode::OpConstant, Some(vec![const_index]));
            }
            Literal::BoolLiteral(v) => {
                match v {
                    true => self.emit(Opcode::OpTrue, None),
                    false => self.emit(Opcode::OpFalse, None),
                };
            }
            Literal::StringLiteral(v) => {
                let lit = Object::String(v);
                let const_index = self.register_constant(&lit) as u16;
                self.emit(Opcode::OpConstant, Some(vec![const_index]));
            }
        };
    }

    pub fn compile_prefix(&mut self, pre: &Prefix, expr: Expr) {
        self.compile_expr(expr);

        match pre {
            Prefix::Not => self.emit(Opcode::OpBang, None),
            Prefix::PrefixPlus => todo!(),
            Prefix::PrefixMinus => self.emit(Opcode::OpMinus, None),
        };
    }

    pub fn compile_infix(&mut self, infix: &Infix, expr1: Expr, expr2: Expr) {
        match infix {
            Infix::LessThan => {
                self.compile_expr(expr2);
                self.compile_expr(expr1);
                self.emit(Opcode::OpGreaterThan, None);
            }
            _ => {
                self.compile_expr(expr1);
                self.compile_expr(expr2);
                match infix {
                    Infix::Plus => self.emit(Opcode::OpAdd, None),
                    Infix::Minus => self.emit(Opcode::OpSub, None),
                    Infix::Divide => self.emit(Opcode::OpDiv, None),
                    Infix::Multiply => self.emit(Opcode::OpMul, None),
                    Infix::Equal => self.emit(Opcode::OpEqual, None),
                    Infix::NotEqual => self.emit(Opcode::OpNotEqual, None),
                    Infix::GreaterThanEqual => todo!(),
                    Infix::LessThanEqual => todo!(),
                    Infix::GreaterThan => self.emit(Opcode::OpGreaterThan, None),
                    _ => unimplemented!(),
                };
            }
        };
    }

    pub fn compile_if(
        &mut self,
        cond: Expr,
        consequence: Vec<Stmt>,
        alternative: Option<Vec<Stmt>>,
    ) {
        self.compile_expr(cond);

        let jump_not_truthy_index = self.emit(Opcode::OpJumpNotTruthy, Some(vec![9999])); // condition => jump to the alternative

        for stmt in consequence {
            self.compile_statement(stmt);
        }

        if self.last_ins_is_pop() {
            self.remove_last_pop()
        }

        let jump_index = self.emit(Opcode::OpJump, Some(vec![9999])); // else => jump to the end of if-else block

        let after_conseq_pos = self.instructions.len();
        self.change_operand(jump_not_truthy_index, after_conseq_pos as u16);

        if alternative == None {
            self.emit(Opcode::OpNull, None);
        } else {
            for stmt in alternative.unwrap() {
                self.compile_statement(stmt)
            }

            if self.last_ins_is_pop() {
                self.remove_last_pop()
            }
        }

        let after_alter_pos = self.instructions.len();
        self.change_operand(jump_index, after_alter_pos as u16);
    }

    pub fn compile_fn(&self, params: Vec<Ident>, body: Vec<Stmt>) {
        todo!()
    }

    pub fn compile_call(&self, fn_exp: Expr, arg: Vec<Expr>) {
        todo!()
    }

    pub fn compile_array(&mut self, exprs: Vec<Expr>) {
        let len = exprs.len();
        for expr in exprs {
            self.compile_expr(expr)
        }

        self.emit(Opcode::OpArray, Some(vec![len as u16]));
    }

    pub fn compile_hash(&mut self, hash_exprs: Vec<(Literal, Expr)>) {
        // TODO: need to find a way to sort so tests wont break
        let len = hash_exprs.len() as u16;
        for (lit, key) in hash_exprs {
            self.compile_literal(lit);
            self.compile_expr(key);
        }

        self.emit(Opcode::OpHash, Some(vec![len * 2]));
    }

    pub fn compile_index(&mut self, array: Expr, index: Expr) {
        self.compile_expr(array);
        self.compile_expr(index);
        self.emit(Opcode::OpIndex, None);
    }

    /// Append obj to constants, return its index as identifier for the OpConstant instruction
    fn register_constant(&mut self, obj: &Object) -> usize {
        self.constants.push(obj.clone());
        self.constants.len() - 1
    }

    /// Generate an instruction and return its position
    fn emit(&mut self, op: Opcode, operands: Option<Vec<u16>>) -> usize {
        let ins = make(op, operands);
        let pos = self.add_instruction(ins);

        self.set_last_instruction(op, pos);

        pos
    }

    fn set_last_instruction(&mut self, op: Opcode, pos: usize) {
        let prev = match self.last_ins {
            Some(ins) => Some(ins),
            None => None,
        };

        let last = EmittedInstruction::new(op, pos);

        self.prev_ins = prev;
        self.last_ins = Some(last);
    }

    /// Add the new instruction to memory, return its position
    fn add_instruction(&mut self, mut ins: Vec<u8>) -> usize {
        let pos_new_ins = self.instructions.len();
        self.instructions.append(&mut ins);
        pos_new_ins
    }

    fn last_ins_is_pop(&self) -> bool {
        self.last_ins.unwrap().opcode == Opcode::OpPop
    }

    /// Remove last instruction, previous instruction becomes the last
    fn remove_last_pop(&mut self) {
        self.instructions.pop();
        self.last_ins = self.prev_ins;
    }

    fn replace_ins(&mut self, pos: usize, new_ins: Vec<u8>) {
        self.instructions[pos..pos + new_ins.len()].copy_from_slice(&new_ins);
    }

    /// Update the instruction at index op_pos with operand
    fn change_operand(&mut self, op_pos: usize, operand: u16) {
        let op = Opcode::from(&self.instructions[op_pos]);
        let new_ins = make(op, Some(vec![operand]));

        self.replace_ins(op_pos, new_ins);
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

#[derive(Debug, Clone, Copy)]
pub struct EmittedInstruction {
    pub opcode: Opcode,
    pub position: usize,
}

impl EmittedInstruction {
    pub fn new(opcode: Opcode, position: usize) -> Self {
        Self { opcode, position }
    }
}

pub mod symbol_table;
#[cfg(test)]
mod test;
