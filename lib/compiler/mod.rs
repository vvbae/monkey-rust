use crate::{
    code::{make, Instructions, Opcode},
    evaluator::object::Object,
    parser::ast::{Expr, Ident, Infix, Literal, Prefix, Program, Stmt},
};

use self::symbol_table::SymbolTable;

pub struct CompilationScope {
    instructions: Instructions,
    last_ins: Option<EmittedInstruction>,
    prev_ins: Option<EmittedInstruction>,
}

pub struct Compiler {
    constants: Vec<Object>,
    symbol_table: SymbolTable,
    scopes: Vec<CompilationScope>,
    scope_index: usize,
}

impl Compiler {
    pub fn new() -> Self {
        let main_scope = CompilationScope {
            instructions: Vec::new(),
            last_ins: None,
            prev_ins: None,
        };

        Self {
            constants: Vec::new(),
            symbol_table: SymbolTable::new(),
            scopes: vec![main_scope],
            scope_index: 0,
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
            Stmt::ReturnStmt(expr) => {
                self.compile_expr(expr);
                self.emit(Opcode::OpReturnValue, None);
            }
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

        if self.last_ins_is(Opcode::OpPop) {
            self.remove_last_pop()
        }

        let jump_index = self.emit(Opcode::OpJump, Some(vec![9999])); // else => jump to the end of if-else block

        let after_conseq_pos = self.current_ins().len();
        self.change_operand(jump_not_truthy_index, after_conseq_pos as u16);

        if alternative == None {
            self.emit(Opcode::OpNull, None);
        } else {
            for stmt in alternative.unwrap() {
                self.compile_statement(stmt)
            }

            if self.last_ins_is(Opcode::OpPop) {
                self.remove_last_pop()
            }
        }

        let after_alter_pos = self.current_ins().len();
        self.change_operand(jump_index, after_alter_pos as u16);
    }

    pub fn compile_fn(&mut self, params: Vec<Ident>, body: Vec<Stmt>) {
        self.enter_scope();

        for stmt in body {
            self.compile_statement(stmt);
        }

        if self.last_ins_is(Opcode::OpPop) {
            self.replace_last_pop_with_return();
        }

        if !self.last_ins_is(Opcode::OpReturnValue) {
            self.emit(Opcode::OpReturn, None);
        }

        let ins = self.leave_scope();

        let compiled_fn = Object::CompiledFn(ins, 0, 0);
        let const_index = self.register_constant(&compiled_fn) as u16;
        self.emit(Opcode::OpConstant, Some(vec![const_index]));
    }

    pub fn compile_call(&mut self, fn_exp: Expr, args: Vec<Expr>) {
        self.compile_expr(fn_exp);
        for arg in args {
            self.compile_expr(arg);
        }

        self.emit(Opcode::OpCall, None);
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
        let prev = match self.scopes[self.scope_index].last_ins {
            Some(ins) => Some(ins),
            None => None,
        };

        let last = EmittedInstruction::new(op, pos);

        self.scopes[self.scope_index].prev_ins = prev;
        self.scopes[self.scope_index].last_ins = Some(last);
    }

    /// Add the new instruction to memory, return its position
    fn add_instruction(&mut self, ins: Vec<u8>) -> usize {
        let mut curr_ins = self.current_ins().clone();
        let pos_new_ins = curr_ins.len();
        curr_ins.extend(ins);

        self.scopes[self.scope_index].instructions = curr_ins;

        pos_new_ins
    }

    fn last_ins_is(&self, op: Opcode) -> bool {
        if self.current_ins().len() == 0 {
            return false;
        }

        self.scopes[self.scope_index].last_ins.unwrap().opcode == op
    }

    /// Remove last instruction, previous instruction becomes the last
    fn remove_last_pop(&mut self) {
        let last = self.scopes[self.scope_index].last_ins.unwrap();
        let prev = self.scopes[self.scope_index].prev_ins.unwrap();

        let old = self.current_ins().clone();
        let new = &old[..last.position];

        self.scopes[self.scope_index].instructions = new.to_vec();
        self.scopes[self.scope_index].last_ins = Some(prev);
    }

    fn replace_ins(&mut self, pos: usize, new_ins: Vec<u8>) {
        let mut ins = self.current_ins().clone();
        ins[pos..pos + new_ins.len()].copy_from_slice(&new_ins);
        self.scopes[self.scope_index].instructions = ins;
    }

    fn replace_last_pop_with_return(&mut self) {
        let last_pos = self.scopes[self.scope_index].last_ins.unwrap().position;
        self.replace_ins(last_pos, make(Opcode::OpReturnValue, None));

        let old = self.scopes[self.scope_index].last_ins.unwrap().position;
        self.scopes[self.scope_index].last_ins = Some(EmittedInstruction {
            opcode: Opcode::OpReturnValue,
            position: old,
        })
    }

    /// Update the instruction at index op_pos with operand
    fn change_operand(&mut self, op_pos: usize, operand: u16) {
        let op = Opcode::from(&self.current_ins()[op_pos]);
        let new_ins = make(op, Some(vec![operand]));

        self.replace_ins(op_pos, new_ins);
    }

    fn current_ins(&self) -> &Instructions {
        &self.scopes[self.scope_index].instructions
    }

    pub fn enter_scope(&mut self) {
        let scope = CompilationScope {
            instructions: Vec::new(),
            last_ins: None,
            prev_ins: None,
        };

        self.scopes.push(scope);
        self.scope_index += 1;
    }

    pub fn leave_scope(&mut self) -> Instructions {
        let ins = self.current_ins().clone();

        self.scopes.pop();
        self.scope_index -= 1;

        ins
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.current_ins().clone(),
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
