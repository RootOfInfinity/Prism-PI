use std::{
    collections::HashMap,
    sync::{Arc, mpsc},
    thread,
};

use super::{
    asm::Instruction,
    ast::{Assignment, DotOp, ExprAST, FunctionAst, Statement},
    tokens::{Literal, Operator, Type},
    typecheck::TypeChecker,
    vm::get_type_size,
};

const INT_NUM: u8 = 1;
const DCML_NUM: u8 = 2;
const BOOL_NUM: u8 = 3;
const STRING_NUM: u8 = 4;
const CALLSTACK_NUM: u8 = 5;
const ARRAY_NUM: u8 = 6;

/// func compiler handles the trenches of the compiling stage
/// the real variable declarations
/// the expressions and statements
/// the real stuff.
pub struct FuncCompiler {
    // consts: Arc<(Vec<u8>, Vec<Type>)>,
    consts: Arc<Vec<u8>>,
    pool: Arc<Vec<String>>,
    ret_types: Arc<HashMap<String, Type>>,
    // ret_types tells it how much mem to allocate to amount_in_stack
    // when a call is given.
    code: Vec<Instruction>,
    func: FunctionAst,
    var_tracker: HashMap<String, (u16, Type)>,
    // SoF will always be zero
    amount_in_stack: u16,
    scoped_vars: Vec<(u16, u16, u16)>,
}

/// Compiler Composer is the manager, it first takes the funcs,
/// splits it up into threads that the func compilers will do seperatly.
/// it also creates consts, the pool, and ret types for each
/// before they are executed. After the compiling employees do their work,
/// it will compose the codes into one stable code without labels, ready for
/// serialization.
/// It will only work with correct code, as error handling is done before this
/// step.
pub struct CompilerComposer {
    // consts: (Vec<u8>, Vec<Type>),
    consts: Vec<u8>,
    pool: Vec<String>,
    funcs: Vec<FunctionAst>,
}

impl FuncCompiler {
    pub fn new(
        // consts: Arc<Vec<u8>, Vec<Type>)>,
        consts: Arc<Vec<u8>>,
        pool: Arc<Vec<String>>,
        ret_types: Arc<HashMap<String, Type>>,
        func: FunctionAst,
    ) -> Self {
        FuncCompiler {
            consts,
            pool,
            ret_types,
            code: Vec::new(),
            func,
            var_tracker: HashMap::new(),
            amount_in_stack: 0,
            scoped_vars: vec![(0, 0, 0)],
        }
    }
    pub fn get_const(consts: &(Vec<u8>), pool: &Vec<String>, lit: &Literal) -> Option<u16> {
        let mut byte_ind: usize = 0;
        loop {
            if byte_ind >= consts.len() {
                break;
            }
            match consts[byte_ind] {
                INT_NUM => {
                    let Literal::Int(find_int) = lit else {
                        byte_ind += get_type_size(INT_NUM);
                        continue;
                    };
                    let int = i32::from_le_bytes(
                        consts[byte_ind + 1..(byte_ind + 1 + size_of::<i32>())]
                            .try_into()
                            .unwrap(),
                    );
                    if *find_int == int {
                        return Some(byte_ind as u16);
                    }
                    byte_ind += get_type_size(INT_NUM);
                }
                DCML_NUM => {
                    let Literal::Dcml(find_dcml) = lit else {
                        byte_ind += get_type_size(DCML_NUM);
                        continue;
                    };
                    let dcml = f64::from_le_bytes(
                        consts[byte_ind + 1..(byte_ind + 1 + size_of::<f64>())]
                            .try_into()
                            .unwrap(),
                    );
                    if *find_dcml == dcml {
                        return Some(byte_ind as u16);
                    }
                    byte_ind += get_type_size(DCML_NUM);
                }
                BOOL_NUM => {
                    let Literal::Bool(find_bool) = lit else {
                        byte_ind += get_type_size(BOOL_NUM);
                        continue;
                    };
                    let boolean = consts[byte_ind + 1] != 0;
                    if *find_bool == boolean {
                        return Some(byte_ind as u16);
                    }
                    byte_ind += get_type_size(BOOL_NUM);
                }
                STRING_NUM => {
                    let Literal::String(find_string) = lit.clone() else {
                        byte_ind += get_type_size(STRING_NUM);
                        continue;
                    };
                    let str_ind = u16::from_le_bytes(
                        consts[byte_ind + 1..(byte_ind + 1 + size_of::<u16>())]
                            .try_into()
                            .unwrap(),
                    ) as usize;
                    if find_string == pool[str_ind] {
                        return Some(byte_ind as u16);
                    }
                    byte_ind += get_type_size(STRING_NUM);
                }
                // CALLSTACK_NUM => (),
                _ => unreachable!(),
            };
        }
        None
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
            Operator::And => Instruction::And,
            Operator::Or => Instruction::Or,
            Operator::Xor => Instruction::Xor,
        });
        if let Operator::NEq = op {
            self.code.push(Instruction::Not);
        }
    }
    fn track_var(&mut self, id: String, typ: Type) {
        // doesn't increment amount_in_stack
        let ind = self.amount_in_stack + typ.size() as u16 - 1;
        self.amount_in_stack += typ.size() as u16;
        self.var_tracker.insert(id, (ind as u16, typ));
    }
    // gets the offset from the top of the var ident passed in
    fn get_var(&self, id: &String) -> (u16, Type) {
        let (varindex, vartype) = self.var_tracker.get(id).unwrap();
        (self.amount_in_stack as u16 - varindex, vartype.to_owned())
    }
    // recursively compiles the ExprAST and pushes it to self.code
    fn compile_expr(&mut self, expr: ExprAST) -> Type {
        match expr {
            ExprAST::Var(x) => {
                let (varoffset, vartype) = self.get_var(&x);
                const PUSH_FROM_STACK: u8 = 0;
                self.code
                    .push(Instruction::Push(PUSH_FROM_STACK, varoffset));
                self.amount_in_stack += vartype.size() as u16;
                return vartype;
            } // find type of the var, and add memsize to stack
            ExprAST::Lit(x) => {
                let ind = FuncCompiler::get_const(&self.consts, &self.pool, &x).unwrap();
                let mem_size = x.get_type().size();
                self.amount_in_stack += mem_size as u16;
                const PUSH_FROM_CONSTS: u8 = 1;
                self.code.push(Instruction::Push(PUSH_FROM_CONSTS, ind));
                return x.get_type();
            }
            ExprAST::BinOp(op, x, y) => {
                let t0 = self.compile_expr(*x);
                let t1 = self.compile_expr(*y);
                self.amount_in_stack -= t0.size() as u16;
                self.amount_in_stack -= t1.size() as u16;
                let endtype = TypeChecker::get_binop_type_panic(t0.to_owned(), t1.to_owned(), &op);
                self.amount_in_stack += endtype.size() as u16;
                // println!(
                //     "After {:#?} {:#?} {:#?}: {}",
                //     t0, op, t1, self.amount_in_stack
                // );
                self.compile_op(op);
                return endtype;
            }
            ExprAST::Call(s, x) => {
                let amount_in_stack_before = self.amount_in_stack;
                for expr in x {
                    self.compile_expr(expr.expr);
                }
                self.amount_in_stack = amount_in_stack_before;
                let datatype = self.ret_types.get(&s).unwrap();
                self.amount_in_stack += datatype.size() as u16;
                self.code.push(Instruction::Call(s));
                return datatype.to_owned();
            }
            ExprAST::Casted(t, expr) => {
                self.amount_in_stack -= self.compile_expr(*expr).size() as u16;
                self.code.push(Instruction::Cast(t.to_owned()));
                self.amount_in_stack += t.size() as u16;

                return t;
            }
            ExprAST::DotOp(dot_op, expr) => {
                self.amount_in_stack -= self.compile_expr(*expr).size() as u16;

                match dot_op {
                    DotOp::Len => {
                        self.code.push(Instruction::ArrLen);
                        self.amount_in_stack += Type::Int.size() as u16;
                        return Type::Int;
                    }
                    DotOp::Pop => {
                        self.code.push(Instruction::ArrPop);
                        return Type::Void;
                    }
                    DotOp::Push(expr) => {
                        self.compile_expr(*expr);
                        self.code.push(Instruction::ArrPush);
                        return Type::Void;
                    }
                }
            }
            ExprAST::Indexed(to_be_indexed, index) => {
                let Type::Array(arr_type) = self.compile_expr(*to_be_indexed) else {
                    unreachable!();
                };
                self.amount_in_stack -= get_type_size(ARRAY_NUM) as u16;
                self.compile_expr(*index);
                self.amount_in_stack -= get_type_size(INT_NUM) as u16;
                self.code.push(Instruction::ArrInd);
                self.amount_in_stack += arr_type.size() as u16;
                return *arr_type;
            }
        }
    }
    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expr(x) => {
                let amount_in_stack_before_expr = self.amount_in_stack;
                self.compile_expr(x.expr);
                self.code.push(Instruction::Pop);
                self.amount_in_stack = amount_in_stack_before_expr;
                // println!("After Expr: {}", self.amount_in_stack);
            }
            Statement::Decl(x) => {
                self.compile_expr(x.val);
                let len = self.scoped_vars.len();
                if let Type::Array(_) = x.typ {
                    self.scoped_vars[len - 1].2 += 1;
                }
                self.scoped_vars[len - 1].1 += x.typ.size() as u16;
                self.track_var(x.ident, x.typ);
                self.scoped_vars[len - 1].0 += 1;
                // println!("After Decl: {}", self.amount_in_stack);
            }
            Statement::Assign(x) => {
                let amount_in_stack_before_expr = self.amount_in_stack;
                self.compile_expr(x.val);
                let vinfo = self.get_var(&x.ident);
                self.code.push(Instruction::Mov(vinfo.0));
                self.code.push(Instruction::Pop);
                self.amount_in_stack = amount_in_stack_before_expr;
                // println!("After Assign: {}", self.amount_in_stack);
            }
            Statement::Return(x) => {
                self.compile_expr(x.expr.expr);
                self.code
                    .push(Instruction::Ret(self.amount_in_stack as u16));
                // it tells the vm to go down by that much in the stack
                // println!("After Return: {}", self.amount_in_stack);
            }
            Statement::If(x) => {
                let amount_in_stack_before_expr = self.amount_in_stack;
                let start_ip = self.code.len();
                self.amount_in_stack -= self.compile_expr(x.cond.expr).size() as u16;
                self.code.push(Instruction::Jnz(format!(
                    "{}-if_{}_else",
                    self.func.name, start_ip
                )));
                // self.code.push(Instruction::Pop);
                // scope start
                self.scoped_vars.push((0, 0, 0));
                for st in x.tcode {
                    self.compile_statement(st);
                }
                self.pop_the_scope();
                // scope end

                self.code.push(Instruction::Jmp(format!(
                    "{}-if_{}_end",
                    self.func.name, start_ip
                )));
                self.code.push(Instruction::Label(format!(
                    "{}-if_{}_else",
                    self.func.name, start_ip
                )));
                // self.code.push(Instruction::Pop);

                self.scoped_vars.push((0, 0, 0));
                for st in x.ecode {
                    self.compile_statement(st);
                }
                self.pop_the_scope();

                self.code.push(Instruction::Label(format!(
                    "{}-if_{}_end",
                    self.func.name, start_ip
                )));
                self.amount_in_stack = amount_in_stack_before_expr;
                // println!("After If: {}", self.amount_in_stack);
            }
            Statement::While(x) => {
                let amount_in_stack_before_expr = self.amount_in_stack;
                let start_ip = self.code.len();
                self.code.push(Instruction::Label(format!(
                    "{}-while_{}",
                    self.func.name, start_ip
                )));
                self.compile_expr(x.cond.expr);
                self.code.push(Instruction::Jnz(format!(
                    "{}-while_{}_end",
                    self.func.name, start_ip
                )));
                // self.code.push(Instruction::Pop);

                self.scoped_vars.push((0, 0, 0));
                for st in x.code {
                    self.compile_statement(st);
                }
                self.pop_the_scope();

                self.code.push(Instruction::Jmp(format!(
                    "{}-while_{}",
                    self.func.name, start_ip
                )));
                self.code.push(Instruction::Label(format!(
                    "{}-while_{}_end",
                    self.func.name, start_ip
                )));
                self.amount_in_stack = amount_in_stack_before_expr;
                // println!("After While: {}", self.amount_in_stack);
            }
        }
    }
    fn pop_the_scope(&mut self) {
        for _ in 0..self.scoped_vars[self.scoped_vars.len() - 1].0 {
            self.code.push(Instruction::Pop);
        }
        for _ in 0..self.scoped_vars[self.scoped_vars.len() - 1].2 {
            self.code.push(Instruction::FreeArr);
        }
        self.amount_in_stack -= self.scoped_vars[self.scoped_vars.len() - 1].1;
        self.scoped_vars.pop();
    }
    pub fn compile(mut self) -> Vec<Instruction> {
        self.code
            .push(Instruction::Label(self.func.name.to_owned()));
        let mut cur_byte_offset = 0;
        let mut from_top = Vec::new();
        for (argstr, argtype) in self.func.params.iter().rev() {
            from_top.push(cur_byte_offset);
            cur_byte_offset += argtype.size();
        }
        let amount_in_stack;
        if self.func.params.len() > 0 {
            amount_in_stack = cur_byte_offset;
        } else {
            amount_in_stack = 0;
        }
        for ind in from_top.iter_mut().rev() {
            *ind = amount_in_stack - *ind;
        }
        for ((id, typ), ind) in self.func.params.iter().zip(from_top.iter()) {
            self.var_tracker
                .insert(id.clone(), (*ind as u16, typ.clone()));
        }
        self.amount_in_stack = amount_in_stack as u16;
        // println!("At start of func: {}", self.amount_in_stack);
        self.code
            .push(Instruction::Fun(self.func.params.len() as u16));
        for st in self.func.code.clone() {
            self.compile_statement(st);
        }
        println!("{} - compiled.", self.func.name);
        self.code
    }
}
impl CompilerComposer {
    pub fn new(funcs: Vec<FunctionAst>) -> Self {
        let mut init = CompilerComposer {
            consts: Vec::new(),
            pool: Vec::new(),
            funcs,
        };
        println!("Creating constants . . .");
        init.create_constants();
        println!("Created Constants");
        init
    }

    fn create_constants(&mut self) {
        for func in self.funcs.clone() {
            self.create_consts_in_codevec(func.code);
        }
    }
    fn create_consts_in_codevec(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::Expr(ex) => {
                    self.create_consts_in_expr(ex.expr);
                }
                Statement::Decl(dec) => {
                    self.create_consts_in_expr(dec.val);
                }
                Statement::Assign(assign) => {
                    self.create_consts_in_expr(assign.val);
                }
                Statement::If(iffy) => {
                    self.create_consts_in_expr(iffy.cond.expr);
                    self.create_consts_in_codevec(iffy.tcode);
                    self.create_consts_in_codevec(iffy.ecode);
                }
                Statement::While(wh) => {
                    self.create_consts_in_expr(wh.cond.expr);
                    self.create_consts_in_codevec(wh.code);
                }
                Statement::Return(ret) => {
                    self.create_consts_in_expr(ret.expr.expr);
                }
            }
        }
    }
    fn create_consts_in_expr(&mut self, expr: ExprAST) {
        match expr {
            ExprAST::Lit(lit) => self.add_const(&lit),
            ExprAST::Var(_) => (),
            ExprAST::BinOp(_, ex0, ex1) => {
                self.create_consts_in_expr(*ex0);
                self.create_consts_in_expr(*ex1);
            }
            ExprAST::Call(_, exprvec) => {
                for expr in exprvec {
                    self.create_consts_in_expr(expr.expr);
                }
            }
            ExprAST::Casted(_, expr) => {
                self.create_consts_in_expr(*expr);
            }
            ExprAST::DotOp(_, expr) => {
                self.create_consts_in_expr(*expr);
            }
            ExprAST::Indexed(ex0, ex1) => {
                self.create_consts_in_expr(*ex0);
                self.create_consts_in_expr(*ex1);
            }
        }
    }
    fn add_const(&mut self, lit: &Literal) {
        match FuncCompiler::get_const(&self.consts, &self.pool, &lit) {
            Some(_) => (),
            None => {
                // self.consts.1.push(lit.get_type());
                match lit {
                    Literal::Int(int) => {
                        self.consts.push(INT_NUM);
                        self.consts.extend_from_slice(&int.to_le_bytes());
                        // used to have identifiers at end for some reason
                    }
                    Literal::Dcml(dcml) => {
                        self.consts.push(DCML_NUM);
                        self.consts.extend_from_slice(&dcml.to_le_bytes());
                    }
                    Literal::Bool(boolean) => {
                        self.consts.push(BOOL_NUM);
                        self.consts.push(*boolean as u8);
                    }
                    Literal::String(string) => {
                        self.pool.push(string.clone());
                        let ind: u16 = self.pool.len() as u16 - 1;
                        self.consts.push(STRING_NUM);
                        self.consts.extend_from_slice(&ind.to_le_bytes());
                    }
                }
            }
        }
    }
    pub fn parallel_compile(&self) -> Vec<Instruction> {
        let mut ret_types = HashMap::new();
        for f in self.funcs.iter() {
            ret_types.insert(f.name.clone(), f.ret_type.clone());
        }
        let arc_consts = Arc::new(self.consts.clone());
        let arc_pool = Arc::new(self.pool.clone());
        let arc_ret = Arc::new(ret_types);
        let (tx, rx) = mpsc::channel();
        let mut handles = Vec::new();
        let mut funcs = self.funcs.clone();
        let mut i = 0;
        let main_func = loop {
            if i >= funcs.len() {
                break None;
            }
            if funcs[i].name == "main".to_string() {
                let mainfunc = funcs.swap_remove(i);
                break Some(mainfunc);
            }
            i += 1;
        };

        println!("Assigning threads to functions");
        for func in funcs {
            let consts = Arc::clone(&arc_consts);
            let pool = Arc::clone(&arc_pool);
            let ret_types = Arc::clone(&arc_ret);
            let tx1 = tx.clone();
            handles.push(thread::spawn(move || {
                let f = &func;
                let f = f.clone();
                let factory = FuncCompiler::new(consts, pool, ret_types, f);
                let inst_vec = factory.compile();
                tx1.send(inst_vec).expect("emergency failure to send");
            }));
        }
        let mut all_instructions: Vec<Instruction> = Vec::new();
        for handle in handles {
            handle.join().unwrap();
            let mut rec = rx.recv().unwrap();
            all_instructions.append(&mut rec);
        }
        if let Some(main) = main_func {
            let factory = FuncCompiler::new(arc_consts, arc_pool, arc_ret, main);
            let mut inst_vec = factory.compile();
            inst_vec.append(&mut all_instructions);
            inst_vec
        } else {
            all_instructions
        }
    }
    pub fn extract_pool_and_consts(self) -> (Vec<String>, Vec<u8>) {
        (self.pool, self.consts)
    }
}
