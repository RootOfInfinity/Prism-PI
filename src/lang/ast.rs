use super::tokens::{Literal, Operator, Type};

pub enum ExprAST {
    Var(String),
    Lit(Literal),
    BinOp((Operator, Box<Expression>, Box<Expression>)),
    Call(String, Vec<ExprAST>),
}

pub enum Statement {
    Expr,
    Decl,
    Assign,
    If,
    While,
    Return,
}

pub struct Declaration {
    pub typ: Type,
    pub ident: String,
    pub ident_loc: Loc,
    pub val: Literal,
    pub val_loc: Loc,
}

pub struct Expression {
    pub expr: ExprAST,
    pub loc: Loc,
}

pub struct Assignment {
    pub ident: String,
    pub ident_loc: Loc,
    pub val: Literal,
    pub val_loc: Loc,
}

pub struct IfBlock {
    pub cond: Expression,
    pub loc: Loc,
    pub code: Vec<Statement>,
}

pub struct WhileBlock {
    pub cond: Expression,
    pub loc: Loc,
    pub code: Vec<Statement>,
}

pub struct Return {
    pub expr: Expression,
    pub loc: Loc,
}

pub struct Loc {
    pub line: u32,
    pub col: u32,
}
