use repl::Repl;

// frontend
mod lexer;
mod tokens;
mod parser;
mod ast;
// backend
mod codegen;
mod asm;
mod assembler;
mod bytecode;
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
    let repl = Repl::new(true, false, false, false, false);
    repl.start();
}
