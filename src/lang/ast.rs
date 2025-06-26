use super::tokens::{Literal, Operator, Type};

#[derive(Clone)]
pub struct FunctionAst {
    pub loc: Loc,
    pub name: String,
    pub params: Vec<(Type, String)>,
    pub code: Vec<Statement>,
    pub ret_type: Type,
}

#[derive(Clone)]
pub enum ExprAST {
    Var(String),
    Lit(Literal),
    BinOp(Operator, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<ExprAST>),
}

#[derive(Clone)]
pub enum Statement {
    Expr(Expression),
    Decl(Declaration),
    Assign(Assignment),
    If(IfBlock),
    While(WhileBlock),
    Return(Return),
}

#[derive(Clone)]
pub struct Declaration {
    pub typ: Type,
    pub ident: String,
    pub ident_loc: Loc,
    pub val: ExprAST,
    pub val_loc: Loc,
}

#[derive(Clone)]
pub struct Expression {
    pub expr: ExprAST,
    pub loc: Loc,
}

#[derive(Clone)]
pub struct Assignment {
    pub ident: String,
    pub ident_loc: Loc,
    pub val: ExprAST,
    pub val_loc: Loc,
}

#[derive(Clone)]
pub struct IfBlock {
    pub cond: Expression,
    pub loc: Loc,
    pub tcode: Vec<Statement>,
    pub ecode: Vec<Statement>,
}

#[derive(Clone)]
pub struct WhileBlock {
    pub cond: Expression,
    pub loc: Loc,
    pub code: Vec<Statement>,
}

#[derive(Clone)]
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
