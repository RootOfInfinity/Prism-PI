use crate::lang::tokens::{Literal, Type};

pub enum ExprAST {
    Var(String),
}

pub enum Statement {
    Expr,
    Decl,
    Assign,
    If,
    While,
}

pub struct Assignment {
    pub typ: Type,
    pub ident: String,
    ident_loc: Loc,
    val: Literal,
    val_loc: Loc,
}

pub struct Loc {
    pub line: u32,
    pub col: u32,
}
