use std::collections::VecDeque;

use crate::lang::ast::Assignment;

use super::ast::{Declaration, Expression, IfBlock, Return, WhileBlock};

use super::tokens::Operator;
use super::{
    ast::{ExprAST, FunctionAst, Loc, Statement},
    errors::{CompileError, ErrorType},
    tokens::{Token, Type},
};

pub struct ParsingMachine {
    cur_tok: (Token, Loc),
    tok_vec: VecDeque<(Token, Loc)>,
    finished: bool,
}
impl ParsingMachine {
    pub fn new(all_tha_tokens: Vec<(Token, Loc)>) -> Self {
        // if all_tha_tokens.len() < 1 {
        //     return None;
        // }
        let mut tok_vec: VecDeque<(Token, Loc)> = all_tha_tokens.into_iter().collect();
        let cur_tok = tok_vec.pop_front().expect("needed one token");
        ParsingMachine {
            cur_tok,
            tok_vec,
            finished: false,
        }
    }
    pub fn append_tok(&mut self, new_stuff: Vec<(Token, Loc)>) {
        for thing in new_stuff {
            self.tok_vec.push_back(thing);
        }
        self.finished = false;
    }
    pub fn parse_all(mut self) -> Result<Vec<FunctionAst>, CompileError> {
        let mut all_funcs = Vec::new();
        while !self.finished {
            all_funcs.push(self.parse_function()?);
        }
        Ok(all_funcs)
    }
    pub fn parse_function(&mut self) -> Result<FunctionAst, CompileError> {
        let (Token::Fun, loc) = &self.cur_tok else {
            return Err(self.err("Did not find keyword 'fun'.".to_string()));
        };
        let loc = loc.to_owned();
        self.eat_tok();
        let Token::Ident(func_ident) = self.cur_tok.0.clone() else {
            return Err(self.err("Expected function name after 'fun' keyword".to_string()));
        };
        self.eat_tok();
        let Token::LeftParen = self.cur_tok.0.clone() else {
            return Err(self.err("Expected left parenthesis after function name".to_string()));
        };
        self.eat_tok();
        // there will be some WEIRD bugs with commas
        // that will be fixed later.
        // this comment will be removed when it is fixed.
        let mut param_vec: Vec<(String, Type)> = Vec::new();
        while !matches!(self.cur_tok.0, Token::RightParen) {
            let Token::DeclareType(typ) = self.cur_tok.0.clone() else {
                return Err(self.err("Expected a type in function parameters".to_string()));
            };
            self.eat_tok(); // eats the type
            let Token::Ident(arg_name) = self.cur_tok.0.clone() else {
                return Err(
                    self.err("Expected ident after type in function parameters".to_string())
                );
            };
            self.eat_tok();
            // eats the ident
            param_vec.push((arg_name, typ));
            match self.cur_tok.0 {
                Token::Comma => {
                    self.eat_tok();
                    continue;
                }
                Token::RightParen => break,
                _ => {
                    return Err(
                        self.err("Expected ',' or ')' after parameter in function".to_string())
                    );
                }
            }
        }
        self.eat_tok();
        let Token::RArrow = self.cur_tok.0 else {
            return Err(self.err("Expected return arrow ('->')".to_string()));
        };
        self.eat_tok();
        let Token::DeclareType(ret_type) = &self.cur_tok.0 else {
            return Err(self.err("Expected return arrow to point to type".to_string()));
        };
        let ret_type = ret_type.to_owned();
        self.eat_tok();
        let block = self.collect_curly_statements()?;
        if let Token::EndOfFile = self.cur_tok.0 {
            self.finished = true;
        }
        Ok(FunctionAst {
            loc,
            name: func_ident,
            params: param_vec,
            code: block,
            ret_type,
        })
    }
    fn collect_curly_statements(&mut self) -> Result<Vec<Statement>, CompileError> {
        if !matches!(self.cur_tok.0, Token::LeftCurly) {
            return Err(self.err("Expected a block".to_string()));
        }
        let stloc = self.cur_tok.1.clone();
        self.eat_tok(); // eat left curly
        let mut state_vec: Vec<Statement> = Vec::new();
        while !matches!(self.cur_tok.0, Token::RightCurly) {
            let statement = self.parse_statement()?;
            state_vec.push(statement);
        }
        self.eat_tok(); // eats right curly
        Ok(state_vec)
    }
    fn parse_statement(&mut self) -> Result<Statement, CompileError> {
        match self.cur_tok.0 {
            Token::DeclareType(_) => self.parse_decl(),
            Token::Ident(_) => match self.peek_tok() {
                Some(&(Token::LeftParen, _)) => Ok(Statement::Expr(self.parse_expression()?)),
                Some(&(Token::Assign, _)) => self.parse_assign(),
                _ => Ok(Statement::Expr(self.parse_expression()?)),
            },
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Return => {
                self.eat_tok();
                let ans = Ok(Statement::Return(Return {
                    expr: self.parse_expression()?,
                    loc: self.cur_tok.1.clone(),
                }));
                let Token::Semicolon = self.cur_tok.0 else {
                    return Err(self.err("Expected semicolon after return statement".to_owned()));
                };
                self.eat_tok();
                ans
            }
            _ => Err(self.err("Unexpected Token".to_owned())),
        }
    }
    fn parse_if(&mut self) -> Result<Statement, CompileError> {
        let loc = self.cur_tok.1.clone();
        self.eat_tok();
        // ate the if
        let cond = self.parse_expression()?;
        let Token::LeftCurly = self.cur_tok.0 else {
            return Err(self.err("Expected block after if conditional".to_string()));
        };
        let ecode;
        let block = self.collect_curly_statements()?;
        if let Token::Else = self.cur_tok.0 {
            self.eat_tok();
            if let Token::LeftCurly = self.cur_tok.0 {
                ecode = self.collect_curly_statements()?;
            } else {
                ecode = vec![self.parse_statement()?];
            }
        } else {
            ecode = Vec::new();
        }
        Ok(Statement::If(IfBlock {
            cond,
            loc,
            tcode: block,
            ecode,
        }))
    }
    fn parse_while(&mut self) -> Result<Statement, CompileError> {
        let loc = self.cur_tok.1.clone();
        self.eat_tok();
        // ate the while
        let cond = self.parse_expression()?;
        let Token::LeftCurly = self.cur_tok.0 else {
            return Err(self.err("Expected block after while conditional".to_string()));
        };
        let block = self.collect_curly_statements()?;
        Ok(Statement::While(WhileBlock {
            cond,
            loc,
            code: block,
        }))
    }

    fn parse_decl(&mut self) -> Result<Statement, CompileError> {
        let Token::DeclareType(typ) = self.cur_tok.0.clone() else {
            unreachable!();
        };
        self.eat_tok(); // eat type
        let (Token::Ident(ident), loc) = self.cur_tok.clone() else {
            return Err(self.err("Expected ident after type declaration".to_string()));
        };
        self.eat_tok(); // eat ident
        if !matches!(self.cur_tok.0, Token::Assign) {
            return Err(self.err(
                "Expected an '=' after declaration (you have to declare and assign in one line)"
                    .to_string(),
            ));
        }
        self.eat_tok();
        let expr = self.parse_expression()?;
        let Token::Semicolon = self.cur_tok.0 else {
            return Err(self.err("Expected a semicolon".to_string()));
        };
        self.eat_tok();
        Ok(Statement::Decl(Declaration {
            typ,
            ident,
            ident_loc: loc,
            val: expr.expr,
            val_loc: expr.loc,
        }))
    }
    fn parse_assign(&mut self) -> Result<Statement, CompileError> {
        let (Token::Ident(name), id_loc) = self.cur_tok.clone() else {
            unreachable!()
        };
        self.eat_tok();
        let Token::Assign = self.cur_tok.0 else {
            unreachable!()
        };
        self.eat_tok();
        let v_loc = self.cur_tok.1.clone();
        let expr = self.parse_expr()?;
        let Token::Semicolon = self.cur_tok.0 else {
            return Err(self.err("Expected a semicolon".to_string()));
        };
        Ok(Statement::Assign(Assignment {
            ident: name,
            ident_loc: id_loc,
            val: expr,
            val_loc: v_loc,
        }))
    }
    fn parse_expression(&mut self) -> Result<Expression, CompileError> {
        let ans = Ok(Expression {
            expr: self.parse_expr()?,
            loc: self.cur_tok.1.clone(),
        });
        // self.eat_tok();
        ans
    }
    fn parse_expr(&mut self) -> Result<ExprAST, CompileError> {
        let lhs = self.parse_primary()?;

        self.parse_rhs(0, lhs)
    }
    fn parse_rhs(&mut self, expr_prior: u32, lhs: ExprAST) -> Result<ExprAST, CompileError> {
        let mut lhs = lhs;
        loop {
            let Token::Op(binop) = self.cur_tok.0.clone() else {
                return Ok(lhs);
            };
            let tok_prior = ParsingMachine::get_priority(&binop);
            if tok_prior < expr_prior {
                return Ok(lhs);
            }
            self.eat_tok(); // eating the operator
            let mut rhs = self.parse_primary()?;
            if let Token::Op(new_binop) = self.cur_tok.0.clone() {
                if ParsingMachine::get_priority(&new_binop) > tok_prior {
                    rhs = self.parse_rhs(tok_prior + 1, rhs)?;
                }
            }
            lhs = ExprAST::BinOp(binop, Box::new(lhs), Box::new(rhs));
        }
    }
    fn parse_ident(&mut self) -> Result<ExprAST, CompileError> {
        let Token::Ident(ident) = &self.cur_tok.0 else {
            unreachable!()
        };
        let ident = ident.clone();
        self.eat_tok();
        if let Token::LeftParen = self.cur_tok.0 {
            self.eat_tok();
            let mut param_vec = Vec::new();
            loop {
                let expr = self.parse_expression()?;
                param_vec.push(expr);
                match self.cur_tok.0 {
                    Token::Comma => {
                        self.eat_tok();
                        continue;
                    }
                    Token::RightParen => {
                        break;
                    }
                    _ => {
                        return Err(
                            self.err("Expected a ',' or ')' in call parameters".to_string())
                        );
                    }
                }
            }
            self.eat_tok();
            return Ok(ExprAST::Call(ident, param_vec));
        } else {
            return Ok(ExprAST::Var(ident));
        }
    }
    fn parse_paren(&mut self) -> Result<ExprAST, CompileError> {
        self.eat_tok(); // the left parenthesis
        let expr = self.parse_expr()?;
        let Token::RightParen = self.cur_tok.0 else {
            return Err(self.err("Expected a closed parenthesis in expression".to_string()));
        };
        return Ok(expr);
    }
    fn parse_primary(&mut self) -> Result<ExprAST, CompileError> {
        match &self.cur_tok.0 {
            Token::Ident(_) => self.parse_ident(),
            Token::Lit(lit) => {
                let ans = Ok(ExprAST::Lit(lit.clone()));
                self.eat_tok();
                ans
            }
            Token::LeftParen => self.parse_paren(),
            _ => {
                Err(self
                    .err("Expected an Identifier, Literal, or '(', got unknown token".to_string()))
            }
        }
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
    fn get_priority(op: &Operator) -> u32 {
        match op {
            Operator::And | Operator::Or | Operator::Xor => 10,
            Operator::LEq
            | Operator::Less
            | Operator::GEq
            | Operator::Greater
            | Operator::Eq
            | Operator::NEq => 20,
            Operator::BAnd | Operator::BOr | Operator::BXor => 30,
            Operator::Add | Operator::Sub => 40,
            Operator::Mult | Operator::Div | Operator::Mod => 50,
        }
    }
    fn err(&self, err_name: String) -> CompileError {
        return CompileError {
            e_type: ErrorType::ParsingError(err_name),
            line: self.cur_tok.1.line,
            col: self.cur_tok.1.col,
        };
    }
}
