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
    And,
    Or,
    Xor,
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

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Dcml(f64),
    Bool(bool),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShortHand {
    AddEq,
    SubEq,
    MultEq,
    DivEq,
    ModEq,
}
