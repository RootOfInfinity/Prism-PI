use std::collections::HashMap;

use super::{
    asm::Instruction,
    ast::{ExprAST, FunctionAst, Statement},
    tokens::{Literal, Type},
};

pub struct ActualFlippinCompiler {
    consts: (Vec<u8>, Vec<Type>),
    pool: Vec<String>,
    code: Vec<Instruction>,
    funcs: Vec<FunctionAst>,
}
impl ActualFlippinCompiler {
    pub fn new(funcs: Vec<FunctionAst>) -> Self {
        ActualFlippinCompiler {
            consts: (vec![], vec![]),
            pool: vec![],
            code: vec![],
            funcs,
        }
    }
    fn is_in_consts(&self, val: &[u8]) -> bool {
        todo!()
    }
    fn get_all_consts(&mut self) {
        for func in self.funcs.clone() {
            self.add_consts_in_vec(func.code);
        }
    }
    fn get_const(&self, lit: Literal) {
        todo!()
    }
    fn add_const_in_expr(&mut self, expr: ExprAST) {
        todo!()
    }
    fn add_consts_in_vec(&mut self, ex_vec: Vec<Statement>) {
        for statement in ex_vec {
            match statement {
                Statement::Expr(x) => self.add_const_in_expr(x.expr),
                Statement::Decl(x) => self.add_const_in_expr(x.val),
                Statement::Assign(x) => self.add_const_in_expr(x.val),
                Statement::If(x) => self.add_consts_in_vec(x.code),
                Statement::While(x) => self.add_consts_in_vec(x.code),
                Statement::Return(x) => self.add_const_in_expr(x.expr.expr),
            }
        }
        todo!()
    }
    fn func_gen(&mut self, func: FunctionAst) {}
    fn compile_expr(&mut self, expr: ExprAST) {
        match expr {
            ExprAST::Var(x) => todo!(),
            ExprAST::Lit(x) => {
                let ind = self.get_const(x);
                // self.code.push();
            }
            _ => todo!(),
        }
    }
}
