pub struct Message {
    pub message_type: MessageType,
    pub message_contents: String,
}
pub enum MessageType {
    FuncName,
    FuncType,
    FuncArgs, // they must type out all args, I'm really struggling over here

    DeclType,
    DeclName,
    DeclExpr,

    AssignName,
    AssignExpr,

    ExprExpr,

    ReturnExpr,

    IfCond,

    IfElseCond,

    WhileCond,

    None,
}
