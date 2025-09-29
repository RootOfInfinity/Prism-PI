use std::fs;

use asm::{Assembler, print_instructions};
use ast::{ExprAST, Expression, FunctionAst, IfBlock, Loc, Statement};
use codegen::{CompilerComposer, FuncCompiler};
use ctrlflow::check_for_returns;
use errors::CompileError;
use lexer::LexEngine;
use parser::ParsingMachine;
use tokens::{Literal, Type};
use typecheck::TypeChecker;
use vm::VM;

// frontend
mod ast;
mod lexer;
mod parser;
pub mod tokens;
// backend
mod asm;
mod bytecode;
mod codegen;
mod vm;
mod wrapped_val;
// optimize
// mod optimizing;
// error handling
pub mod errors;
// semantic analysis
mod array;
mod ctrlflow;
mod typecheck;
// debugging
mod repl;

pub fn run_lang_test(args: Vec<String>) {
    // run tests for lang
    println!("lang stuff");
    println!("Getting path");
    let path = args[2].clone();
    println!("Got path: {}", path);
    let raw = fs::read_to_string(path).expect("File not found");
    // println!("RAW CODE:\n{}", raw);
    let lex = LexEngine::new(raw);
    let toks = lex.lex_all().unwrap();
    // println!("TOKENS:\n{:#?}", toks);
    let parser = ParsingMachine::new(toks);
    let ast = parser.parse_all().unwrap();
    // println!("AST:\n{:#?}", ast);
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
    let instructions = compiler.parallel_compile();
    print_instructions(&instructions);
    let bytecode = Assembler::new(instructions).assemble();
    let (pool, consts) = compiler.extract_pool_and_consts();
    println!("Time to RUN!");
    println!("EXECUTE ORDER 66!");
    let mut virtual_machine = VM::new(pool, consts, bytecode);
    let end_val = virtual_machine.execute_order_66();
    println!("The end value was {}", end_val);
}

pub fn run_code(code: String) -> Result<i32, Vec<CompileError>> {
    let mut errvec: Vec<CompileError> = Vec::new();
    let lexer = LexEngine::new(code);
    let toks = match lexer.lex_all() {
        Ok(tokens) => tokens,
        Err(e) => {
            errvec.push(e);
            return Err(errvec);
        }
    };
    let parser = ParsingMachine::new(toks);
    let ast = match parser.parse_all() {
        Ok(a) => a,
        Err(e) => {
            errvec.push(e);
            return Err(errvec);
        }
    };
    match check_for_returns(ast.to_owned()) {
        Ok(()) => (),
        Err(mut e) => {
            errvec.append(&mut e);
        }
    }
    if errvec.len() > 0 {
        return Err(errvec);
    }
    match TypeChecker::new(ast.to_owned()).check_all() {
        Ok(()) => (),
        Err(mut e) => {
            errvec.append(&mut e);
        }
    }
    if !ast
        .iter()
        .map(|func| func.name.clone())
        .collect::<Vec<String>>()
        .contains(&"main".to_string())
    {
        errvec.push(CompileError {
            e_type: errors::ErrorType::ParsingError("No main func bro".to_string()),
            line: 0,
            col: 0,
        });
    }
    if errvec.len() > 0 {
        return Err(errvec);
    }

    let compiler = CompilerComposer::new(ast);
    let instructions = compiler.parallel_compile();
    let bytecode = Assembler::new(instructions).assemble();
    let (pool, consts) = compiler.extract_pool_and_consts();
    Ok(VM::new(pool, consts, bytecode).execute_order_66())
}
