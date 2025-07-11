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
    Else,
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
    // Logical (right now, it is just for precidence)
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
    Int,       // 1
    Dcml,      // 2
    Bool,      // 3
    String,    // 4
    CallStack, // 5
}
impl Type {
    //size in bytes including tag at end
    pub fn size(&self) -> usize {
        (match self {
            Type::Int => 4,
            Type::Dcml => 8,
            Type::Bool => 1,
            Type::String => 2,
            Type::CallStack => 4,
        }) + 1
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
