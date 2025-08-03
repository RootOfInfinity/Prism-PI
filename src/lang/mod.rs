use std::fs;

use asm::print_instructions;
use ast::{ExprAST, Expression, FunctionAst, IfBlock, Loc, Statement};
use codegen::{CompilerComposer, FuncCompiler};
use ctrlflow::check_for_returns;
use lexer::LexEngine;
use parser::ParsingMachine;
use tokens::{Literal, Type};
use typecheck::TypeChecker;

// frontend
mod ast;
mod lexer;
mod parser;
mod tokens;
// backend
mod asm;
mod bytecode;
mod codegen;
mod vm;
mod wrapped_val;
// optimize
// mod optimizing;
// error handling
mod errors;
// semantic analysis
mod ctrlflow;
mod typecheck;
// debugging
mod repl;

pub fn run_lang_test(args: Vec<String>) {
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
    // let func_runner = CompilerComposer::new(vec![func]);
    // print_instructions(&func_runner.parallel_compile());
    println!("Getting path");
    let path = args[2].clone();
    println!("Got path: {}", path);
    let raw = fs::read_to_string(path).expect("File not found");
    println!("RAW CODE:\n{}", raw);
    let lex = LexEngine::new(raw);
    let toks = lex.lex_all().unwrap();
    // println!("TOKENS:\n{:#?}", toks);
    let parser = ParsingMachine::new(toks);
    let ast = parser.parse_all().unwrap();
    println!("AST:\n{:#?}", ast);
    match check_for_returns(ast.to_owned()) {
        Ok(()) => println!("Control Flow diagram reports NO ERRORS!"),
        Err(errvec) => {
            println!("Control Flow diagram reports RETURN ERRORS.\n{:#?}", errvec);
            return;
        }
    }
    let type_checker = TypeChecker::new(ast.to_owned());
    if let Err(vec) = type_checker.check_all() {
        println!("Type Check reports TYPE ERRORS.\n{:#?}", vec);
        return;
    } else {
        println!("Type Check reports NO ERRORS!");
    }
    let compiler = CompilerComposer::new(ast);
    print_instructions(&compiler.parallel_compile());
}
