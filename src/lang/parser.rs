use std::collections::VecDeque;

use crate::lang::tokens::Token;

pub struct ParsingMachine {
    cur_tok: Token,
    tok_vec: VecDeque<Token>,
    finished: bool,
}
impl ParsingMachine {
    pub fn new(all_tha_tokens: Vec<Token>) -> Option<Self> {
        if all_tha_tokens.len() < 1 {
            return None;
        }
        let mut tok_vec: VecDeque<Token> = all_tha_tokens.into_iter().collect();
        let cur_tok = tok_vec.pop_front().unwrap();
        Some(ParsingMachine {
            cur_tok,
            tok_vec,
            finished: false,
        })
    }
    pub fn append_tok(&mut self, new_stuff: Vec<Token>) {
        for thing in new_stuff {
            self.tok_vec.push_back(thing);
        }
        self.finished = false;
    }

    fn eat_tok(&mut self) {
        self.cur_tok = match self.tok_vec.pop_front() {
            Some(x) => x,
            None => Token::EndOfFile,
        };
    }
    fn peek_tok(&mut self) -> Option<&Token> {
        if self.tok_vec.len() > 0 {
            Some(&self.tok_vec[0])
        } else {
            None
        }
    }
}
