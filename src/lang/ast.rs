use super::tokens::{Literal, Operator, Type};

pub struct FunctionAst {
    loc: Loc,
    name: String,
    params: Vec<(Type, String)>,
    code: Vec<Statement>,
    ret_type: Type,
}

pub enum ExprAST {
    Var(String),
    Lit(Literal),
    BinOp(Operator, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<ExprAST>),
}

pub enum Statement {
    Expr(Expression),
    Decl(Declaration),
    Assign(Assignment),
    If(IfBlock),
    While(WhileBlock),
    Return(Return),
}

pub struct Declaration {
    pub typ: Type,
    pub ident: String,
    pub ident_loc: Loc,
    pub val: ExprAST,
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
