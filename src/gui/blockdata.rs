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
    Expression(String),
    // Expression(VisualExpr),
    Return(String),
    // Return(VisualExpr),
    If(IfBlk),
    IfElse(IfBlk, BlockID),
    While(WhileBlk),
    None,
}

pub type ExprID = u64;

#[derive(Clone, Debug)]
pub struct VisualExpr {
    vtype: VExprType,
    id: String,
    // id: ExprID,
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
    set_to: String,
    // set_to: ExprID,
}

#[derive(Clone, Debug)]
pub struct IfBlk {
    pub cond: String,
    // pub cond: ExprID,
    pub if_stuff: BlockID,
}

#[derive(Clone, Debug)]
pub struct WhileBlk {
    pub cond: String,
    // pub cond: ExprID,
    pub while_stuff: BlockID,
}

// Helper structs for VisualExpr

#[derive(Clone, Debug)]
pub struct BinOperator {
    op_enum: BinOp,
    left: String,
    // left: ExprID,
    right: String,
    // right: ExprID,
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

pub type World = (
    HashMap<BlockID, Block>,
    HashMap<ExprID, VisualExpr>,
    u64,
    u64,
);

pub trait WorldManipulation {
    // new Blocks
    // fn new_func(&mut self);
    // fn new_decl(&mut self);
    // fn new_assign(&mut self);
    // fn new_expression(&mut self);
    // fn new_return(&mut self);
    // fn new_if(&mut self);
    // fn new_ifelse(&mut self);
    // fn new_while(&mut self);

    // manipulating Blocks
    fn attach(&mut self, block: BlockID, attaching: BlockID);
    fn rem(&mut self, block_deleted: BlockID);
    fn detach(&mut self, detaching: BlockID);
    fn attach_if(&mut self, block: BlockID, attaching: BlockID);
    fn attach_else(&mut self, block: BlockID, attaching: BlockID);
    fn attach_while(&mut self, block: BlockID, attaching: BlockID);
    fn move_block(&mut self, block: BlockID, x: u64, y: u64);

    // fn affix_binop(&mut self, block: BlockID, expr: ExprID);
    // fn affix_literal(&mut self, block: BlockID, expr: ExprID);
    // fn affix_variable(&mut self, block: BlockID, expr: ExprID);
    // fn affix_var_in_assign(&mut self, block: BlockID, var: ExprID);

    // /// turns the type of decl and func to said type
    // fn change_type(&mut self, block: BlockID, btype: Type);
    // /// changes the name of decl block
    // fn change_name_decl_block(&mut self, block: BlockID, name: String);

    // // new Exprs
    // fn new_operator(&mut self, op: BinOp);
    // fn new_literal(&mut self, val: Value);
    // fn new_variable(&mut self, name: String);

    // // affix to op Expr
    // fn op_affix_left(&mut self, op: ExprID, id: ExprID);
    // fn op_affix_right(&mut self, op: ExprID, id: ExprID);

    // // change values of the Exprs
    // fn change_lit_val(&mut self, expr: ExprID, val: Value);
    // fn change_var_name(&mut self, expr: ExprID, name: String);
}

impl WorldManipulation for World {
    fn attach(&mut self, block: BlockID, attaching: BlockID) {
        self.0.get_mut(&block).unwrap().next = attaching;
    }
    fn rem(&mut self, block_deleted: BlockID) {
        fn kill_everything_in_sight(cur: BlockID, world: &World) -> Vec<BlockID> {
            let mut murdervec = vec![cur];
            let real_block = world.0.get(&cur).unwrap();
            if real_block.id != 0 {
                murdervec.append(&mut kill_everything_in_sight(real_block.next, world));
            }
            match real_block.btype {
                BlockType::If(IfBlk {
                    cond: _,
                    if_stuff: x,
                }) => {
                    murdervec.append(&mut kill_everything_in_sight(x, world));
                }
                BlockType::IfElse(
                    IfBlk {
                        cond: _,
                        if_stuff: x,
                    },
                    y,
                ) => {
                    murdervec.append(&mut kill_everything_in_sight(x, world));
                    murdervec.append(&mut kill_everything_in_sight(y, world));
                }
                BlockType::While(WhileBlk {
                    cond: _,
                    while_stuff: x,
                }) => {
                    murdervec.append(&mut kill_everything_in_sight(x, world));
                }
                _ => (),
            }

            murdervec
        }
        let murdervec = kill_everything_in_sight(block_deleted, &self);
        // time for the slaughter!
        for death_row_inmate in murdervec {
            self.0.remove(&death_row_inmate);
        }
    }
    fn detach(&mut self, detaching: BlockID) {
        for block in self.0.values_mut() {
            if block.next == detaching {
                block.next = 0;
            } else {
                match block.btype {
                    BlockType::If(IfBlk {
                        cond: _,
                        if_stuff: ref mut x,
                    }) if *x == detaching => *x = 0,
                    BlockType::IfElse(
                        IfBlk {
                            cond: _,
                            if_stuff: ref mut x,
                        },
                        _,
                    ) if *x == detaching => *x = 0,
                    BlockType::IfElse(_, ref mut x) if *x == detaching => *x = 0,
                    BlockType::While(WhileBlk {
                        cond: _,
                        while_stuff: ref mut x,
                    }) if *x == detaching => *x = 0,
                    _ => (),
                }
            }
        }
        self.0.get_mut(&detaching).unwrap().is_root = true;
    }
    fn attach_if(&mut self, block: BlockID, attaching: BlockID) {
        match self.0.get_mut(&block).unwrap().btype {
            BlockType::If(IfBlk {
                cond: _,
                if_stuff: ref mut x,
            }) => *x = attaching,
            BlockType::IfElse(
                IfBlk {
                    cond: _,
                    if_stuff: ref mut x,
                },
                _,
            ) => *x = attaching,
            _ => (),
        }
    }
    fn attach_else(&mut self, block: BlockID, attaching: BlockID) {
        match self.0.get_mut(&block).unwrap().btype {
            BlockType::IfElse(_, ref mut x) => *x = attaching,
            _ => (),
        }
    }
    fn attach_while(&mut self, block: BlockID, attaching: BlockID) {
        match self.0.get_mut(&block).unwrap().btype {
            BlockType::While(WhileBlk {
                cond: _,
                while_stuff: ref mut x,
            }) => *x = attaching,
            _ => (),
        }
    }
    fn move_block(&mut self, block: BlockID, x: u64, y: u64) {
        let real_block = self.0.get_mut(&block).unwrap();
        let is_root = real_block.is_root;
        real_block.loc.0 = x;
        real_block.loc.1 = y;
        // add stuff for dealing with attachment and detachment.
        if !is_root {
            self.detach(block);
        }
    }
}
