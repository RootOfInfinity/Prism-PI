use asm::print_instructions;
use ast::{ExprAST, Expression, FunctionAst, IfBlock, Loc, Statement};
use codegen::{CompilerComposer, FuncCompiler};
use repl::Repl;
use tokens::{Literal, Type};

// frontend
mod ast;
mod lexer;
mod parser;
mod tokens;
// backend
mod asm;
mod assembler;
mod bytecode;
mod codegen;
mod vm;
// optimize
// mod optimizing;
// error handling
mod errors;
// debugging
mod repl;

pub fn run_lang_test() {
    // run tests for lang
    println!("lang stuff");
    let func = FunctionAst {
        loc: Loc::new(0, 0),
        name: "main".to_string(),
        params: vec![],
        code: vec![
            Statement::If(IfBlock {
                cond: Expression {
                    expr: ExprAST::Lit(Literal::Bool(true)),
                    loc: Loc::new(0, 0),
                },
                loc: Loc::new(0, 0),
                tcode: vec![Statement::Expr(Expression {
                    expr: ExprAST::Lit(Literal::Int(32)),
                    loc: Loc::new(0, 0),
                })],
                ecode: Vec::new(),
            }),
            Statement::Return(ast::Return {
                expr: Expression {
                    expr: ExprAST::BinOp(
                        tokens::Operator::Sub,
                        Box::new(ExprAST::Lit(Literal::Int(5))),
                        Box::new(ExprAST::Lit(Literal::Int(4))),
                    ),
                    loc: Loc::new(0, 0),
                },
                loc: Loc::new(0, 0),
            }),
        ],
        ret_type: Type::Int,
    };
    let func_runner = CompilerComposer::new(vec![func]);
    print_instructions(&func_runner.parallel_compile());
}
