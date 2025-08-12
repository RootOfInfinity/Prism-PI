use super::tokens::{Literal, Operator, Type};

#[derive(Clone, Debug)]
pub struct FunctionAst {
    pub loc: Loc,
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub code: Vec<Statement>,
    pub ret_type: Type,
}

#[derive(Clone, Debug)]
pub enum ExprAST {
    Var(String),
    Lit(Literal),
    BinOp(Operator, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<Expression>),
    Casted(Type, Box<ExprAST>),
    DotOp(DotOp, Box<ExprAST>),
    Indexed(Box<ExprAST>, Box<ExprAST>),
}

#[derive(Clone, Debug)]
pub enum DotOp {
    Len,
    Push(Box<ExprAST>),
    Pop,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Expr(Expression),
    Decl(Declaration),
    Assign(Assignment),
    If(IfBlock),
    While(WhileBlock),
    Return(Return),
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub typ: Type,
    pub ident: String,
    pub ident_loc: Loc,
    pub val: ExprAST,
    pub val_loc: Loc,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expr: ExprAST,
    pub loc: Loc,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub ident: String,
    pub ident_loc: Loc,
    pub val: ExprAST,
    pub val_loc: Loc,
}

#[derive(Clone, Debug)]
pub struct IfBlock {
    pub cond: Expression,
    pub loc: Loc,
    pub tcode: Vec<Statement>,
    pub ecode: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct WhileBlock {
    pub cond: Expression,
    pub loc: Loc,
    pub code: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Return {
    pub expr: Expression,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Loc {
    pub line: u32,
    pub col: u32,
}
impl Loc {
    pub fn new(line: u32, col: u32) -> Self {
        Loc { line, col }
    }
}
