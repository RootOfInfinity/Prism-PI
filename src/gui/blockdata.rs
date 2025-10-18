use std::collections::HashMap;

// position will be determined outside of this data structure
// this is mainly for holding the relationship between blocks
// as such, many enums will have a None option for when the user
// hasn't put in their value yet.
// Of course, to turn it into text, we will need no 'None's eventually.

pub type BlockID = u64;

#[derive(Clone, Debug)]
pub struct Block {
    pub btype: BlockType,
    pub id: BlockID,
    pub next: BlockID,
    pub loc: (u64, u64),
    pub is_root: bool,
}
#[derive(Clone, Debug)]
pub enum BlockType {
    FuncStart(Func),
    Declaration(Type, Assign),
    Assignment(Assign),
    Expression(VisualExpr),
    Return(VisualExpr),
    If(IfBlk),
    IfElse(IfBlk, BlockID),
    While(WhileBlk),
    None,
}

pub type ExprID = u64;

#[derive(Clone, Debug)]
pub struct VisualExpr {
    vtype: VExprType,
    id: ExprID,
}
#[derive(Clone, Debug)]
pub enum VExprType {
    BinOp(BinOperator),
    Literal(Type, Value),
    Variable(String),
    None,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Dcml(f64),
    Bool(bool),
    None,
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Dcml,
    Bool,
    None,
}

// Helper structs for BlockType

#[derive(Clone, Debug)]
pub struct Func {
    fname: String,
    ret_type: Type,
    args: Vec<(String, Type)>,
}

#[derive(Clone, Debug)]
pub struct Assign {
    vname: String,
    set_to: ExprID,
}

#[derive(Clone, Debug)]
pub struct IfBlk {
    cond: ExprID,
    if_stuff: BlockID,
}

#[derive(Clone, Debug)]
pub struct WhileBlk {
    cond: ExprID,
    while_stuff: BlockID,
}

// Helper structs for VisualExpr

#[derive(Clone, Debug)]
pub struct BinOperator {
    op_enum: BinOp,
    left: ExprID,
    right: ExprID,
}

#[derive(Clone, Debug)]
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
// There needs to be an instance of Arc<Mutex<BlockType>> to use with the UI
// Point is, we can make the backend of the blocks seperate from the UI of the blocks.
// Just like with the text language.

pub type World = (HashMap<BlockID, Block>, HashMap<ExprID, VisualExpr>);

pub trait WorldManipulation {
    // new Blocks
    fn new_func(&mut self);
    fn new_decl(&mut self);
    fn new_assign(&mut self);
    fn new_expression(&mut self);
    fn new_return(&mut self);
    fn new_if(&mut self);
    fn new_ifelse(&mut self);
    fn new_while(&mut self);

    // manipulating Blocks
    fn attach(&mut self, block: BlockID, attaching: BlockID);
    fn rem(&mut self, block_deleted: BlockID);
    fn detach(&mut self, detaching: BlockID);
    fn attach_if(&mut self, block: BlockID, attaching: BlockID);
    fn attach_else(&mut self, block: BlockID, attaching: BlockID);
    fn attach_while(&mut self, block: BlockID, attaching: BlockID);

    fn affix_binop(&mut self, block: BlockID, expr: ExprID);
    fn affix_literal(&mut self, block: BlockID, expr: ExprID);
    fn affix_variable(&mut self, block: BlockID, expr: ExprID);
    fn affix_var_in_assign(&mut self, block: BlockID, var: ExprID);

    /// turns the type of decl and func to said type
    fn change_type(&mut self, block: BlockID, btype: Type);
    /// changes the name of decl block
    fn change_name_decl_block(&mut self, block: BlockID, name: String);

    // new Exprs
    fn new_operator(&mut self, op: BinOp);
    fn new_literal(&mut self, val: Value);
    fn new_variable(&mut self, name: String);

    // affix to op Expr
    fn op_affix_left(&mut self, op: ExprID, id: ExprID);
    fn op_affix_right(&mut self, op: ExprID, id: ExprID);

    // change values of the Exprs
    fn change_lit_val(&mut self, expr: ExprID, val: Value);
    fn change_var_name(&mut self, expr: ExprID, name: String);
}

// impl WorldManipulation for World {}
