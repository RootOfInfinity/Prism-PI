use std::io::{self, Write};

use crate::lang::{lexer::LexEngine, tokens::Token};

pub struct Repl {
    lex_output: bool,
    ast_output: bool,
    ir_output: bool,
    bytecode_text_output: bool,
    execution: bool,
}
impl Repl {
    pub fn new(
        lex_output: bool,
        ast_output: bool,
        ir_output: bool,
        bytecode_text_output: bool,
        execution: bool,
    ) -> Self {
        Repl {
            lex_output,
            ast_output,
            ir_output,
            bytecode_text_output,
            execution,
        }
    }
    pub fn start(&self) -> ! {
        println!("-Initializing REPL-");
        print!(":> ");
        io::stdout().flush();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf);
        let mut lex = LexEngine::new(buf);
        let mut tok_vec = Vec::new();
        loop {
            match lex.get_tok() {
                Ok(Token::EndOfFile) => break,
                Ok(x) => tok_vec.push(x),
                Err(e) => eprintln!("Error: {:#?} at {}:{}", e.e_type, e.line, e.col),
            }
        }
        if self.lex_output {
            for tok in tok_vec {
                println!("{:#?}", tok);
            }
        }
        loop {
            print!("\n:> ");
            io::stdout().flush();
            let mut buf = String::new();
            io::stdin().read_line(&mut buf);
            lex.append_string(buf);
            let mut tok_vec = Vec::new();
            loop {
                match lex.get_tok() {
                    Ok(Token::EndOfFile) => break,
                    Ok(x) => tok_vec.push(x),
                    Err(e) => eprintln!("Error: {:#?} at {}:{}", e.e_type, e.line, e.col),
                }
            }
            if self.lex_output {
                for tok in tok_vec {
                    println!("{:#?}", tok);
                }
            }
        }
    }
}
