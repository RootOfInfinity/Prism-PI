use std::collections::HashMap;

use super::{
    asm::Instruction,
    ast::{ExprAST, FunctionAst, Statement},
    tokens::{Literal, Operator, Type},
};

/// func compiler handles the trenches of the compiling stage
/// the real variable declarations
/// the expressions and statements
/// the real stuff.
struct FuncCompiler<'a> {
    consts: &'a (Vec<u8>, Vec<Type>),
    pool: &'a Vec<String>,
    ret_types: &'a HashMap<String, Type>,
    // ret_types tells it how much mem to allocate to stack_len
    // when a call is given.
    code: Vec<Instruction>,
    func: FunctionAst,
    var_tracker: HashMap<String, (u16, Type)>,
    // SoF will always be zero
    stack_len: u16,
}

/// Compiler Composer is the manager, it first takes the funcs,
/// splits it up into threads that the func compilers will do seperatly.
/// it also creates consts, the pool, and ret types for each
/// before they are executed. After the compiling employees do their work,
/// it will compose the codes into one stable code without labels, ready for
/// serialization.
/// It will only work with correct code, as error handling is done before this
/// step.
struct CompilerComposer {
    consts: (Vec<u8>, Vec<Type>),
    pool: Vec<String>,
    code: Vec<Instruction>,
    funcs: Vec<FunctionAst>,
}

impl<'a> FuncCompiler<'a> {
    pub fn new(
        consts: &'a (Vec<u8>, Vec<Type>),
        pool: &'a Vec<String>,
        ret_types: &'a HashMap<String, Type>,
        func: FunctionAst,
    ) -> Self {
        FuncCompiler {
            consts,
            pool,
            ret_types,
            code: Vec::new(),
            func,
            var_tracker: HashMap::new(),
            stack_len: 0,
        }
    }
    fn get_const(&self, lit: &Literal) -> u16 {
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
                    if *find_int == int {
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
                    if *find_dcml == dcml {
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
                    if *find_bool == boolean {
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
    fn track_var(&mut self, id: String, typ: Type) {
        // doesn't increment stack_top
        let ind = self.stack_len;
        self.var_tracker.insert(id, (ind as u16, typ));
    }
    // gets the offset from the top of the var ident passed in
    fn get_var(&self, id: &String) -> (u16, Type) {
        let vinfo = self.var_tracker.get(id).unwrap();
        (self.stack_len as u16 - vinfo.0, vinfo.1.clone())
    }
    // recursively compiles the ExprAST and pushes it to self.code
    fn compile_expr(&mut self, expr: ExprAST) {
        match expr {
            ExprAST::Var(x) => {
                let vinfo = self.get_var(&x);
                self.code.push(Instruction::Push(0, vinfo.0));
                self.stack_len += vinfo.1.size() as u16;
            } // find type of the var, and add memsize to stack
            ExprAST::Lit(x) => {
                let ind = self.get_const(&x);
                let mem_size = x.get_type().size();
                self.stack_len += mem_size as u16;
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
                self.stack_len += self.ret_types.get(&s).unwrap().size() as u16;
                self.code.push(Instruction::Call(s));
            }
        }
    }
    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expr(x) => {
                let stack_len_before_expr = self.stack_len;
                self.compile_expr(x.expr);
                self.code.push(Instruction::Pop);
                self.stack_len = stack_len_before_expr;
            }
            Statement::Decl(x) => {
                self.compile_expr(x.val);
                self.track_var(x.ident, x.typ);
            }
            Statement::Assign(x) => {
                let stack_len_before_expr = self.stack_len;
                self.compile_expr(x.val);
                let vinfo = self.get_var(&x.ident);
                self.code.push(Instruction::Mov(vinfo.0));
                self.code.push(Instruction::Pop);
                self.stack_len = stack_len_before_expr;
            }
            Statement::Return(x) => {
                self.compile_expr(x.expr.expr);
                self.code.push(Instruction::Ret(self.stack_len as u16));
            }
            Statement::If(x) => {
                let stack_len_before_expr = self.stack_len;
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
                self.stack_len = stack_len_before_expr;
            }
            Statement::While(x) => {
                let stack_len_before_expr = self.stack_len;
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
                self.stack_len = stack_len_before_expr;
            }
        }
    }
}
impl CompilerComposer {
    pub fn new(funcs: Vec<FunctionAst>) -> Self {
        CompilerComposer {
            consts: (Vec::new(), Vec::new()),
            pool: Vec::new(),
            code: Vec::new(),
            funcs,
        }
    }
}
