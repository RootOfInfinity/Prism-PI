// position will be determined outside of this data structure
// this is mainly for holding the relationship between blocks
// as such, many enums will have a None option for when the user
// hasn't put in their value yet.
// Of course, to turn it into text, we will need no 'None's eventually.

pub enum Block {
    FuncStart(Func),
    Declaration(Type, Assign),
    Assignment(Assign),
    Expression(VisualExpr),
    Return(VisualExpr),
    If(IfBlk),
    IfElse(IfBlk, Vec<Block>),
    While(WhileBlk),
}

pub enum VisualExpr {
    BinOp(BinOperator),
    Literal(Type, Value),
    Variable(String),
    None,
}

pub enum Value {
    Int(i32),
    Dcml(f64),
    Bool(bool),
    None,
}

pub enum Type {
    Int,
    Dcml,
    Bool,
    None,
}

// Helper structs for Block

pub struct Func {
    fname: String,
    ret_type: Type,
    args: Vec<(String, Type)>,
    blocks: Vec<Block>,
}

pub struct Assign {
    vname: String,
    set_to: VisualExpr,
}

pub struct IfBlk {
    cond: VisualExpr,
    if_stuff: Vec<Block>,
}

pub struct WhileBlk {
    cond: VisualExpr,
    while_stuff: Vec<Block>,
}

// Helper structs for VisualExpr

pub struct BinOperator {
    op_enum: BinOp,
    left: Box<VisualExpr>,
    right: Box<VisualExpr>,
}

pub enum BinOp {
    // Basic arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Comparison
    Eq,
    Neq,
    Ls,
    Gr,
    Le,
    Ge,

    // Boolean algebra
    And,
    Or,
    Xor,
}

// And that is all the data, now we need algorithms to enact on it
// We also need to integrate ui actions to code
// There needs to be an instance of Arc<Mutex<Block>> to use with the UI
// Point is, we can make the backend of the blocks seperate from the UI of the blocks.
// Just like with the text language.
