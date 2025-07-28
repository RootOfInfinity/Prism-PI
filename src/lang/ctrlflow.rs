use crate::lang::errors::ErrorType;

use super::{
    ast::{FunctionAst, IfBlock, Statement, WhileBlock},
    errors::CompileError,
};
use std::rc::Rc;

struct BasicBlock {
    code: Vec<Statement>,
    inputs: OneOrTwo<BasicBlock>,
    returns: bool,
}

enum OneOrTwo<T> {
    None,
    One(Rc<T>),
    Two(Rc<T>, Rc<T>),
}

pub fn check_for_returns(ast: Vec<FunctionAst>) -> Result<(), Vec<CompileError>> {
    let mut errvec = Vec::new();
    for func in ast {
        let correct = check_for_ret(Rc::new(create_basic_blocks(func.code, OneOrTwo::None)));
        if !correct {
            errvec.push(CompileError {
                e_type: ErrorType::ControlFlowError(format!(
                    "The function '{}' might not return",
                    func.name
                )),
                line: func.loc.line,
                col: func.loc.col,
            });
        }
    }
    if errvec.len() == 0 {
        Ok(())
    } else {
        Err(errvec)
    }
}

fn create_basic_blocks(funccode: Vec<Statement>, inputs: OneOrTwo<BasicBlock>) -> BasicBlock {
    let mut buf: Vec<Statement> = Vec::new();
    let mut cur_ret = false;
    let mut cur_inputs: OneOrTwo<BasicBlock> = inputs;
    for statement in &funccode {
        match statement {
            Statement::If(x) => {
                let block = BasicBlock {
                    code: buf,
                    inputs: cur_inputs,
                    returns: cur_ret,
                };
                cur_inputs = create_if_basic(Rc::new(block), x.to_owned());
                buf = Vec::new();
                cur_ret = false;
            }
            Statement::While(x) => {
                let rcblock = Rc::new(BasicBlock {
                    code: buf,
                    inputs: cur_inputs,
                    returns: cur_ret,
                });
                let other_block = create_while_basic(Rc::clone(&rcblock), x.to_owned());
                cur_inputs = OneOrTwo::Two(Rc::clone(&rcblock), Rc::new(other_block));
                buf = Vec::new();
                cur_ret = false;
            }
            Statement::Return(x) => {
                cur_ret = true;
                buf.push(Statement::Return(x.to_owned()));
            }
            x => buf.push(x.to_owned()),
        }
    }
    return BasicBlock {
        code: buf,
        inputs: cur_inputs,
        returns: cur_ret,
    };
}

fn create_if_basic(input: Rc<BasicBlock>, if_blk: IfBlock) -> OneOrTwo<BasicBlock> {
    let t_block = create_basic_blocks(if_blk.tcode.to_owned(), OneOrTwo::One(Rc::clone(&input)));
    let e_block = create_basic_blocks(if_blk.ecode.to_owned(), OneOrTwo::One(Rc::clone(&input)));
    return OneOrTwo::Two(Rc::new(t_block), Rc::new(e_block));
}

fn create_while_basic(inputs: Rc<BasicBlock>, while_blk: WhileBlock) -> BasicBlock {
    return create_basic_blocks(while_blk.code.to_owned(), OneOrTwo::One(Rc::clone(&inputs)));
}

fn check_for_ret(blk: Rc<BasicBlock>) -> bool {
    if blk.returns {
        return true;
    } else {
        return match blk.inputs {
            OneOrTwo::None => false,
            OneOrTwo::One(ref x) => check_for_ret(Rc::clone(x)),
            OneOrTwo::Two(ref x, ref y) => {
                check_for_ret(Rc::clone(x)) && check_for_ret(Rc::clone(y))
            }
        };
    }
}
