use std::collections::VecDeque;

use crate::lang::tokens::Token;

pub struct ParsingMachine {
    cur_tok: Token,
    tok_vec: VecDeque<Token>,
}
