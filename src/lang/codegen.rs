use core::panic;
use std::collections::HashMap;

use super::{
    asm::Instruction,
    ast::{ExprAST, FunctionAst, Statement},
    tokens::{Literal, Operator, Type},
};

pub struct ActualFlippinCompiler {
    consts: (Vec<u8>, Vec<Type>),
    pool: Vec<String>,
    code: Vec<Instruction>,
    funcs: Vec<FunctionAst>,
    cur_func_name: String,
    cur_if: u32,
    cur_while: u32,
}
impl ActualFlippinCompiler {
    pub fn new(funcs: Vec<FunctionAst>) -> Self {
        ActualFlippinCompiler {
            consts: (vec![], vec![]),
            pool: vec![],
            code: vec![],
            funcs,
            cur_func_name: String::new(),
            cur_if: 0,
            cur_while: 0,
        }
    }
    fn get_const(&self, lit: Literal) -> u16 {
        let mut byte_ind: usize = 0;
        for t in self.consts.1.iter() {
            match t {
                Type::Int => {
                    let Literal::Int(find_int) = lit else {
                        byte_ind += t.size();
                        continue;
                    };
                    let int = i32::from_le_bytes(
                        self.consts.0[byte_ind..(byte_ind + t.size())]
                            .try_into()
                            .unwrap(),
                    );
                    if find_int == int {
                        return byte_ind as u16;
                    }
                    byte_ind += t.size();
                }
                Type::Dcml => {
                    let Literal::Dcml(find_dcml) = lit else {
                        byte_ind += t.size();
                        continue;
                    };
                    let dcml = f64::from_le_bytes(
                        self.consts.0[byte_ind..(byte_ind + t.size())]
                            .try_into()
                            .unwrap(),
                    );
                    if find_dcml == dcml {
                        return byte_ind as u16;
                    }
                    byte_ind += t.size();
                }
                Type::Bool => {
                    let Literal::Bool(find_bool) = lit else {
                        byte_ind += t.size();
                        continue;
                    };
                    let boolean = self.consts.0[byte_ind] != 0;
                    if find_bool == boolean {
                        return byte_ind as u16;
                    }
                    byte_ind += t.size();
                }
                Type::String => {
                    let Literal::String(find_string) = lit.clone() else {
                        byte_ind += t.size();
                        continue;
                    };
                    let str_ind = u16::from_le_bytes(
                        self.consts.0[byte_ind..(byte_ind + t.size())]
                            .try_into()
                            .unwrap(),
                    ) as usize;
                    if find_string == self.pool[str_ind] {
                        return byte_ind as u16;
                    }
                    byte_ind += t.size();
                }
            };
        }
        unreachable!("Couldn't find the constant needed");
    }

    // compiles an operator into a instruction like (Operator::Add) -> (Instruction::Add)
    // and pushes it into the code
    fn compile_op(&mut self, op: Operator) {
        self.code.push(match op {
            Operator::Add => Instruction::Add,
            Operator::Sub => Instruction::Sub,
            Operator::Mult => Instruction::Mul,
            Operator::Div => Instruction::Div,
            Operator::Mod => Instruction::Mod,
            Operator::Eq => Instruction::Eq,
            Operator::NEq => Instruction::Eq,
            Operator::Less => Instruction::L,
            Operator::LEq => Instruction::Le,
            Operator::Greater => Instruction::G,
            Operator::GEq => Instruction::Ge,
            Operator::BAnd => Instruction::And,
            Operator::BOr => Instruction::Or,
            Operator::BXor => Instruction::Xor,
        });
        if let Operator::NEq = op {
            self.code.push(Instruction::Not);
        }
    }
    // generate a function
    fn func_gen(&mut self, func: FunctionAst) {}
    fn track_var(&mut self) {}
    // gets the offset from the top of the var ident passed in
    fn get_var(&self, id: String) -> u16 {
        todo!()
    }
    // gets the offset from the top of the start of the current function
    fn get_sof_off(&self) -> u16 {
        todo!()
    }
    // recursively compiles the ExprAST and pushes it to self.code
    fn compile_expr(&mut self, expr: ExprAST) {
        match expr {
            ExprAST::Var(x) => todo!(),
            ExprAST::Lit(x) => {
                let ind = self.get_const(x);
                self.code.push(Instruction::Push(1, ind));
            }
            ExprAST::BinOp(op, x, y) => {
                self.compile_expr(*x);
                self.compile_expr(*y);
                self.compile_op(op);
            }
            ExprAST::Call(s, x) => {
                for expr in x {
                    self.compile_expr(expr);
                }
                self.code.push(Instruction::Call(s));
            }
        }
    }
    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expr(x) => {
                self.compile_expr(x.expr);
                self.code.push(Instruction::Pop);
            }
            Statement::Decl(x) => {
                self.compile_expr(x.val);
                self.track_var()
            }
            Statement::Assign(x) => {
                self.compile_expr(x.val);
                let offset = self.get_var(x.ident);
                self.code.push(Instruction::Mov(offset));
                self.code.push(Instruction::Pop);
            }
            Statement::Return(x) => {
                self.compile_expr(x.expr.expr);
                self.code.push(Instruction::Ret(self.get_sof_off()));
            }
            Statement::If(x) => {
                let start_ip = self.code.len();
                self.compile_expr(x.cond.expr);
                self.code
                    .push(Instruction::Jz(format!("if_{}_else", start_ip)));
                self.code.push(Instruction::Pop);
                for st in x.tcode {
                    self.compile_statement(st);
                }
                self.code
                    .push(Instruction::Jmp(format!("if_{}_end", start_ip)));
                self.code
                    .push(Instruction::Label(format!("if_{}_else", start_ip)));
                self.code.push(Instruction::Pop);
                for st in x.ecode {
                    self.compile_statement(st);
                }
                self.code
                    .push(Instruction::Label(format!("if_{}_end", start_ip)));
            }
            Statement::While(x) => {
                let start_ip = self.code.len();
                self.code
                    .push(Instruction::Label(format!("while_{}", start_ip)));
                self.compile_expr(x.cond.expr);
                self.code
                    .push(Instruction::Jz(format!("while_{}_end", start_ip)));
                self.code.push(Instruction::Pop);
                for st in x.code {
                    self.compile_statement(st);
                }
                self.code
                    .push(Instruction::Jmp(format!("while_{}", start_ip)));
                self.code
                    .push(Instruction::Label(format!("while_{}_end", start_ip)));
            }
        }
    }
}
