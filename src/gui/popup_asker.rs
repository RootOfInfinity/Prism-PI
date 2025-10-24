use slint::ToSharedString;

use super::slint_generatedMainWindow;

pub struct Message {
    pub message_type: slint_generatedMainWindow::MessageType,
    pub message_contents: String,
}
// pub enum MessageType {
//     FuncName,
//     FuncType,
//     FuncArgs, // they must type out all args, I'm really struggling over here

//     DeclType,
//     DeclName,
//     DeclExpr,

//     AssignName,
//     AssignExpr,

//     ExprExpr,

//     ReturnExpr,

//     IfCond,

//     IfElseCond,

//     WhileCond,

//     None,
// }

pub fn ask_popup(message: Message, window: &slint::Weak<slint_generatedMainWindow::MainWindow>) {
    window.unwrap().invoke_freestyle_popup_appear(
        message.message_type,
        message.message_contents.to_shared_string(),
    );
}
