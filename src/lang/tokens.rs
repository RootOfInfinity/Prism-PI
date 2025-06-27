#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Lit(Literal),
    ShortHand(ShortHand),
    Op(Operator),
    Semicolon,
    Fun,
    DeclareType(Type),
    While,
    If,
    LeftParen,
    RightParen,
    LeftBrack,
    RightBrack,
    LeftCurly,
    RightCurly,
    Comma,
    Assign,
    Return,
    RArrow,
    EndOfFile,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    // basic math
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    // equality
    Eq,
    NEq,
    // comparison
    Less,
    LEq,
    Greater,
    GEq,
    // Logical
    // And,
    // Or,
    // Xor,
    // Bitwise
    BAnd,
    BOr,
    BXor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Dcml,
    Bool,
    String,
}
impl Type {
    //size in bytes
    pub fn size(&self) -> usize {
        match self {
            Type::Int => 4,
            Type::Dcml => 8,
            Type::Bool => 1,
            Type::String => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Dcml(f64),
    Bool(bool),
    String(String),
}
impl Literal {
    // gets type of lit
    pub fn get_type(&self) -> Type {
        match self {
            Literal::Int(_) => Type::Int,
            Literal::Dcml(_) => Type::Dcml,
            Literal::Bool(_) => Type::Bool,
            Literal::String(_) => Type::String,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShortHand {
    AddEq,
    SubEq,
    MultEq,
    DivEq,
    ModEq,
}
