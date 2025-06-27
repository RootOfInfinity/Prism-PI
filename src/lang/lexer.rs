use std::{collections::VecDeque, iter::Peekable, str::Chars};

use crate::lang::{
    errors::{CompileError, ErrorType},
    tokens::{Literal, Operator, ShortHand, Token, Type},
};

use super::ast::Loc;

pub struct LexEngine {
    line: u32,
    col: u32,
    cur_char: char,
    char_collect: VecDeque<char>,
    finished: bool,
}
impl LexEngine {
    pub fn new(langstr: String) -> Self {
        let mut char_vec = langstr.chars().collect::<VecDeque<char>>();
        let first_char = match char_vec.pop_front() {
            Some(x) => x,
            None => ' ',
        };
        LexEngine {
            line: 0,
            col: 0,
            cur_char: first_char,
            char_collect: char_vec,
            finished: false,
        }
    }
    pub fn append_string(&mut self, new_stuff: String) {
        for thing in new_stuff.chars() {
            self.char_collect.push_back(thing);
        }
        self.finished = false;
    }
    pub fn get_tok(&mut self) -> Result<(Token, Loc), CompileError> {
        // remove whitespace
        if self.finished {
            return Ok((Token::EndOfFile, Loc::new(self.line, self.col)));
        }
        while self.cur_char.is_whitespace() {
            self.eat_char();
            if self.finished {
                return Ok((Token::EndOfFile, Loc::new(self.line, self.col)));
            }
        }
        // remove comments
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.eat_char();
                if self.finished {
                    return Ok((
                        Token::EndOfFile,
                        Loc {
                            line: self.line,
                            col: self.col,
                        },
                    ));
                }
            }
            self.eat_char();
        }
        // find ident
        if self.is_alpha(false) {
            let mut ident_string = String::new();
            ident_string.push(self.cur_char);
            self.eat_char();
            while self.is_alpha(true) {
                ident_string.push(self.cur_char);
                self.eat_char();
            }
            return Ok((
                match ident_string.as_str() {
                    "fun" => Token::Fun,
                    "int" => Token::DeclareType(Type::Int),
                    "string" => Token::DeclareType(Type::String),
                    "dcml" => Token::DeclareType(Type::Dcml),
                    "bool" => Token::DeclareType(Type::Bool),
                    "if" => Token::If,
                    "while" => Token::While,
                    "true" => Token::Lit(Literal::Bool(true)),
                    "false" => Token::Lit(Literal::Bool(false)),
                    "return" => Token::Return,
                    x => Token::Ident(x.to_owned()),
                },
                Loc {
                    line: self.line,
                    col: self.col,
                },
            ));
        }
        if self.is_numeric() {
            let mut has_point = false;
            let mut num_string = String::new();
            if self.cur_char == '.' {
                has_point = true;
                num_string.push('0');
            }
            num_string.push(self.cur_char);
            self.eat_char();
            while self.is_numeric() {
                if self.cur_char == '.' {
                    if has_point {
                        return Err(CompileError::new(
                            ErrorType::LexingError,
                            self.line,
                            self.col,
                        ));
                    } else {
                        has_point = true;
                    }
                }
                num_string.push(self.cur_char);
                self.eat_char();
            }
            if has_point {
                return match num_string.parse::<f64>() {
                    Ok(x) => Ok((
                        Token::Lit(Literal::Dcml(x)),
                        Loc {
                            line: self.line,
                            col: self.col,
                        },
                    )),
                    Err(_) => unreachable!(),
                };
            } else {
                return match num_string.parse::<i32>() {
                    Ok(x) => Ok((
                        Token::Lit(Literal::Int(x)),
                        Loc {
                            line: self.line,
                            col: self.col,
                        },
                    )),
                    Err(_) => unreachable!(),
                };
            }
        }
        if self.cur_char == '"' {
            self.eat_char();
            let mut string_lit = String::new();
            // let mut last_char = '"';
            while self.cur_char != '"' && !self.finished
            /* || (self.cur_char == '"' && last_char == '\\') */
            {
                if self.cur_char == '\\' {
                    println!("No way, hey");
                    let next_char = match self.peek_char() {
                        Some(x) => x,
                        None => {
                            return Err(CompileError::new(
                                ErrorType::LexingError,
                                self.line,
                                self.col,
                            ));
                        }
                    };
                    match next_char {
                        '\\' => string_lit.push('\\'),
                        'n' => string_lit.push('\n'),
                        '"' => string_lit.push('"'),
                        _ => {
                            self.eat_char();
                            return Err(CompileError::new(
                                ErrorType::LexingError,
                                self.line,
                                self.col,
                            ));
                        }
                    }
                    self.eat_char();
                    self.eat_char();
                } else {
                    string_lit.push(self.cur_char);
                    // last_char = self.cur_char;
                    self.eat_char();
                }
            }
            if self.finished {
                return Err(CompileError::new(
                    ErrorType::LexingError,
                    self.line,
                    self.col,
                ));
            }
            self.eat_char();
            return Ok((
                Token::Lit(Literal::String(string_lit)),
                Loc {
                    line: self.line,
                    col: self.col,
                },
            ));
        }
        if self.is_part_of_symbol() {
            let mut sym_string = String::new();
            sym_string.push(self.cur_char);
            self.eat_char();
            while self.is_part_of_symbol() {
                sym_string.push(self.cur_char);
            }
            match LexEngine::get_symbol(&sym_string) {
                Some(x) => {
                    return Ok((
                        x,
                        Loc {
                            line: self.line,
                            col: self.col,
                        },
                    ));
                }
                None => {
                    return Err(CompileError::new(
                        ErrorType::LexingError,
                        self.line,
                        self.col,
                    ));
                }
            }
        }
        eprintln!(
            "Did not know what to do with {}. I got no clue ngl",
            self.cur_char
        );
        self.eat_char();
        return Err(CompileError::new(
            ErrorType::LexingError,
            self.line,
            self.col,
        ));
    }
    fn eat_char(&mut self) {
        if self.cur_char == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        match self.char_collect.pop_front() {
            Some(x) => self.cur_char = x,
            None => {
                self.cur_char = ' ';
                self.finished = true;
            }
        }
    }
    fn peek_char(&mut self) -> Option<&char> {
        if self.char_collect.len() > 0 {
            Some(&self.char_collect[0])
        } else {
            None
        }
    }

    fn is_alpha(&self, numeric: bool) -> bool {
        if (self.cur_char >= 'a' && self.cur_char <= 'z')
            || (self.cur_char >= 'A' && self.cur_char <= 'Z')
            || (numeric && self.cur_char >= '0' && self.cur_char <= '9')
        {
            true
        } else {
            false
        }
    }
    fn is_numeric(&self) -> bool {
        if (self.cur_char >= '0' && self.cur_char <= '9') || self.cur_char == '.' {
            true
        } else {
            false
        }
    }
    fn is_part_of_symbol(&self) -> bool {
        match self.cur_char {
            '+' | '-' | '*' | '/' | '%' => true,
            '=' | '!' => true,
            '<' | '>' => true,
            '&' | '|' | '^' => true,
            ',' | ';' => true,
            '(' | ')' | '[' | ']' | '{' | '}' => true,
            _ => false,
        }
    }
    fn get_symbol(sym_str: &str) -> Option<Token> {
        match sym_str {
            "+" => Some(Token::Op(Operator::Add)),
            "-" => Some(Token::Op(Operator::Sub)),
            "*" => Some(Token::Op(Operator::Mult)),
            "/" => Some(Token::Op(Operator::Div)),
            "%" => Some(Token::Op(Operator::Mod)),
            "==" => Some(Token::Op(Operator::Eq)),
            "!=" => Some(Token::Op(Operator::NEq)),
            "<" => Some(Token::Op(Operator::Less)),
            "<=" => Some(Token::Op(Operator::LEq)),
            ">" => Some(Token::Op(Operator::Greater)),
            ">=" => Some(Token::Op(Operator::GEq)),
            // "&&" => Some(Token::Op(Operator::And)),
            // "||" => Some(Token::Op(Operator::Or)),
            // "^^" => Some(Token::Op(Operator::Xor)),
            "&" => Some(Token::Op(Operator::BAnd)),
            "|" => Some(Token::Op(Operator::BOr)),
            "^" => Some(Token::Op(Operator::BXor)),
            "=" => Some(Token::Assign),
            "+=" => Some(Token::ShortHand(ShortHand::AddEq)),
            "-=" => Some(Token::ShortHand(ShortHand::SubEq)),
            "*=" => Some(Token::ShortHand(ShortHand::MultEq)),
            "/=" => Some(Token::ShortHand(ShortHand::DivEq)),
            "%=" => Some(Token::ShortHand(ShortHand::ModEq)),
            ";" => Some(Token::Semicolon),
            "," => Some(Token::Comma),
            "(" => Some(Token::LeftParen),
            ")" => Some(Token::RightParen),
            "[" => Some(Token::LeftBrack),
            "]" => Some(Token::RightBrack),
            "{" => Some(Token::LeftCurly),
            "}" => Some(Token::RightCurly),
            "->" => Some(Token::RArrow),

            _ => None,
        }
    }
}
