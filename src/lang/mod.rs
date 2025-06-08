use repl::Repl;

// frontend
mod ast;
mod lexer;
mod parser;
mod tokens;
// backend
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
    let repl = Repl::new(true, false, false, false, false);
    repl.start();
}
