use std::collections::VecDeque;

use super::{
    ast::{FunctionAst, Loc},
    errors::{CompileError, ErrorType},
    tokens::{Token, Type},
};

pub struct ParsingMachine {
    cur_tok: (Token, Loc),
    tok_vec: VecDeque<(Token, Loc)>,
    finished: bool,
}
impl ParsingMachine {
    pub fn new(all_tha_tokens: Vec<(Token, Loc)>) -> Option<Self> {
        if all_tha_tokens.len() < 1 {
            return None;
        }
        let mut tok_vec: VecDeque<(Token, Loc)> = all_tha_tokens.into_iter().collect();
        let cur_tok = tok_vec.pop_front().unwrap();
        Some(ParsingMachine {
            cur_tok,
            tok_vec,
            finished: false,
        })
    }
    pub fn append_tok(&mut self, new_stuff: Vec<(Token, Loc)>) {
        for thing in new_stuff {
            self.tok_vec.push_back(thing);
        }
        self.finished = false;
    }
    pub fn parse_function(&mut self) -> Result<FunctionAst, CompileError> {
        let Token::Fun = self.cur_tok.0 else {
            return Err(CompileError::new(
                ErrorType::ParsingError,
                self.cur_tok.1.line,
                self.cur_tok.1.col,
            ));
        };
        self.eat_tok();
        let Token::Ident(func_ident) = self.cur_tok.0.clone() else {
            return Err(CompileError::new(
                ErrorType::ParsingError,
                self.cur_tok.1.line,
                self.cur_tok.1.col,
            ));
        };
        self.eat_tok();
        let Token::LeftParen = self.cur_tok.0.clone() else {
            return Err(CompileError::new(
                ErrorType::ParsingError,
                self.cur_tok.1.line,
                self.cur_tok.1.col,
            ));
        };
        // there will be some WEIRD bugs with commas
        // that will be fixed later.
        // this comment will be removed when it is fixed.
        let mut peram_vec: Vec<(Type, String)> = Vec::new();
        while !matches!(self.cur_tok.0, Token::RightParen) {
            let Token::DeclareType(typ) = self.cur_tok.0.clone() else {
                return Err(CompileError::new(
                    ErrorType::ParsingError,
                    self.cur_tok.1.line,
                    self.cur_tok.1.col,
                ));
            };
            self.eat_tok(); // eats the type
            let Token::Ident(arg_name) = self.cur_tok.0.clone() else {
                return Err(CompileError::new(
                    ErrorType::ParsingError,
                    self.cur_tok.1.line,
                    self.cur_tok.1.col,
                ));
            };
            self.eat_tok(); // eats the ident
            peram_vec.push((typ, arg_name));
            match self.cur_tok.0 {
                Token::Comma => {
                    self.eat_tok();
                    continue;
                }
                Token::RightParen => break,
                _ => {
                    return Err(CompileError::new(
                        ErrorType::ParsingError,
                        self.cur_tok.1.line,
                        self.cur_tok.1.col,
                    ));
                }
            }
        }

        todo!()
    }

    fn eat_tok(&mut self) {
        self.cur_tok = match self.tok_vec.pop_front() {
            Some(x) => x,
            None => (Token::EndOfFile, Loc { line: 0, col: 0 }),
        };
    }
    fn peek_tok(&mut self) -> Option<&(Token, Loc)> {
        if self.tok_vec.len() > 0 {
            Some(&self.tok_vec[0])
        } else {
            None
        }
    }
}
