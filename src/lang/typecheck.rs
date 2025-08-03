use std::{collections::HashMap, fmt::Error, mem::discriminant};

use crate::lang::{ast::Assignment, tokens::Operator};

use super::{
    ast::{ExprAST, Expression, FunctionAst, IfBlock, Loc, Statement},
    errors::{CompileError, ErrorType},
    tokens::Type,
};

pub struct TypeChecker {
    ast: Vec<FunctionAst>,
    funcmap: HashMap<String, (Vec<Type>, Type)>,
    errors: Vec<CompileError>,
}
impl TypeChecker {
    pub fn new(ast: Vec<FunctionAst>) -> Self {
        TypeChecker {
            ast,
            funcmap: HashMap::new(),
            errors: Vec::new(),
        }
    }
    pub fn check_all(mut self) -> Result<(), Vec<CompileError>> {
        for func in &self.ast {
            let mut arg_types = Vec::new();
            for arg in &func.params {
                arg_types.push(arg.1.to_owned());
            }
            self.funcmap
                .insert(func.name.to_owned(), (arg_types, func.ret_type.to_owned()));
        }
        for func in self.ast.to_owned() {
            _ = self.check_func(func.to_owned());
        }
        if self.errors.len() == 0 {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
    fn check_expr(
        &mut self,
        ex: ExprAST,
        loc: Loc,
        varmap: &HashMap<String, Type>,
    ) -> Result<Type, CompileError> {
        match ex {
            ExprAST::Var(ref s) => Ok(varmap.get(s).unwrap().clone()),
            ExprAST::Lit(ref lit) => Ok(lit.get_type()),
            ExprAST::BinOp(ref op, ex0, ex1) => Ok(
                match (
                    op,
                    self.check_expr(*ex0.to_owned(), loc.to_owned(), varmap)?,
                    self.check_expr(*ex1.to_owned(), loc.to_owned(), varmap)?,
                ) {
                    (
                        Operator::Add
                        | Operator::Sub
                        | Operator::Mult
                        | Operator::Div
                        | Operator::Mod,
                        Type::Int,
                        Type::Int,
                    ) => Type::Int,
                    // Through research, I have found Rust does not add ints to floats
                    // (
                    //     Operator::Add | Operator::Sub | Operator::Mult | Operator::Div,
                    //     Type::Dcml,
                    //     Type::Int,
                    // ) => Type::Dcml,
                    // (
                    //     Operator::Add | Operator::Sub | Operator::Mult | Operator::Div,
                    //     Type::Int,
                    //     Type::Dcml,
                    // ) => Type::Dcml,
                    (
                        Operator::Add | Operator::Sub | Operator::Mult | Operator::Div,
                        Type::Dcml,
                        Type::Dcml,
                    ) => Type::Dcml,
                    (Operator::Eq | Operator::NEq, x, y)
                        if discriminant(&x) == discriminant(&y) =>
                    {
                        Type::Bool
                    }
                    (
                        Operator::Less | Operator::LEq | Operator::Greater | Operator::GEq,
                        Type::Int,
                        Type::Int,
                    ) => Type::Bool,
                    (
                        Operator::Less | Operator::LEq | Operator::Greater | Operator::GEq,
                        Type::Dcml,
                        Type::Dcml,
                    ) => Type::Bool,
                    (Operator::And | Operator::Or | Operator::Xor, Type::Bool, Type::Bool) => {
                        Type::Bool
                    }
                    (Operator::BAnd | Operator::BOr | Operator::BXor, Type::Int, Type::Int) => {
                        Type::Int
                    }
                    // BitAnd is not implemented for f64
                    // (Operator::BAnd | Operator::BOr | Operator::BXor, Type::Dcml, Type::Dcml) => {
                    //     Type::Dcml
                    // }
                    (Operator::BAnd | Operator::BOr | Operator::BXor, Type::Bool, Type::Bool) => {
                        Type::Bool
                    }
                    (op, t0, t1) => {
                        let err = self.err_binop(&loc, op, t0, t1);
                        self.add_err(err.clone());
                        return Err(err);
                    }
                },
            ),
            ExprAST::Call(ref s, ref exprs) => {
                let (inputs, output) = self.funcmap.get(s).unwrap().to_owned();
                if inputs.len() != exprs.len() {
                    let err = self.err(
                        &loc,
                        &format!(
                            "Expected {} arguments in call to function '{}', got {} args",
                            inputs.len(),
                            s,
                            exprs.len()
                        ),
                    );
                    self.add_err(err.clone());
                    return Err(err);
                }
                for (expect_in, actual_in) in inputs.clone().iter().zip(exprs.iter()) {
                    let actual_in =
                        self.check_expr(actual_in.expr.to_owned(), loc.to_owned(), varmap)?;
                    if discriminant(expect_in) != discriminant(&actual_in) {
                        let err = self.err_func(&loc, s, &expect_in, &actual_in);
                        self.add_err(err.clone());
                        return Err(err);
                    }
                }
                Ok(output.to_owned())
            }
        }
    }
    fn check_statement(
        &mut self,
        statement: Statement,
        varmap: &mut HashMap<String, Type>,
        ret_type: &Type, // TODO ADD TYPE CHECK FOR RETURN TYPES
    ) -> Result<(), CompileError> {
        match statement {
            Statement::Expr(expression) => {
                self.check_expr(expression.expr, expression.loc, &varmap)?;
                Ok(())
            }
            Statement::Decl(declaration) => {
                let expr_ret_type =
                    self.check_expr(declaration.val, declaration.val_loc, &varmap)?;
                if discriminant(&declaration.typ) != discriminant(&expr_ret_type) {
                    let err = CompileError {
                        e_type: ErrorType::TypeError(format!(
                            "Tried to set variable '{}' of type '{:#?}' to type of '{:#?}'",
                            &declaration.ident, declaration.typ, expr_ret_type
                        )),
                        line: declaration.ident_loc.line,
                        col: declaration.ident_loc.col,
                    };
                    self.errors.push(err.to_owned());
                    return Err(err);
                }
                if varmap.contains_key(&declaration.ident) {
                    let err = self.err(
                        &declaration.ident_loc,
                        &format!(
                            "Cannot declare the same named variable '{}' twice in the same scope (Shadowing is not allowed).",
                            declaration.ident
                        ),
                    );
                    self.add_err(err.to_owned());
                    return Err(err);
                };

                varmap.insert(declaration.ident, declaration.typ);
                Ok(())
            }
            Statement::Assign(assignment) => {
                let expr_ret_type = self.check_expr(assignment.val, assignment.val_loc, &varmap)?;
                let Some(actual_type) = varmap.get(&assignment.ident) else {
                    let err = self.err(
                        &assignment.ident_loc,
                        &format!(
                            "Could not find variable '{}' in current scope",
                            assignment.ident
                        ),
                    );
                    self.add_err(err.to_owned());
                    return Err(err);
                };
                if discriminant(actual_type) != discriminant(&expr_ret_type) {
                    let err = CompileError {
                        e_type: ErrorType::TypeError(format!(
                            "Tried to set variable '{}' of type '{:#?}' to type of '{:#?}'",
                            &assignment.ident, actual_type, expr_ret_type
                        )),
                        line: assignment.ident_loc.line,
                        col: assignment.ident_loc.col,
                    };
                    self.errors.push(err.to_owned());
                    return Err(err);
                }

                Ok(())
            }
            Statement::If(ifblock) => {
                let condcheck = self.check_expr(ifblock.cond.expr, ifblock.cond.loc, &varmap);
                let truecheck = self.check_block(ifblock.tcode, &varmap, ret_type);
                let falsecheck = self.check_block(ifblock.ecode, &varmap, ret_type);
                condcheck?;
                truecheck?;
                falsecheck
            }
            Statement::While(whileblock) => {
                let condcheck = self.check_expr(whileblock.cond.expr, whileblock.cond.loc, &varmap);
                let blockcheck = self.check_block(whileblock.code, &varmap, ret_type);
                condcheck?;
                blockcheck
            }
            Statement::Return(returnblock) => {
                self.check_expr(returnblock.expr.expr, returnblock.expr.loc, &varmap)?;
                Ok(())
            }
        }
    }
    fn check_block(
        &mut self,
        block: Vec<Statement>,
        varmap: &HashMap<String, Type>,
        ret_type: &Type,
    ) -> Result<(), CompileError> {
        let mut varmap = varmap.to_owned();
        for statement in block {
            self.check_statement(statement, &mut varmap, ret_type)?;
        }
        Ok(())
    }
    fn check_func(&mut self, func: FunctionAst) -> Result<(), CompileError> {
        let mut varmap = HashMap::new();
        for arg in func.params {
            varmap.insert(arg.0, arg.1);
        }
        for statement in func.code {
            self.check_statement(statement, &mut varmap, &func.ret_type)?;
        }
        Ok(())
    }
    fn err(&self, loc: &Loc, custom_message: &String) -> CompileError {
        CompileError {
            e_type: ErrorType::TypeError(custom_message.clone()),
            line: loc.line,
            col: loc.col,
        }
    }
    fn err_func(
        &self,
        loc: &Loc,
        func_name: &String,
        expected: &Type,
        actual: &Type,
    ) -> CompileError {
        CompileError {
            e_type: ErrorType::TypeError(format!(
                "Expected type '{:#?}', got type '{:#?}' in function '{}'",
                expected, actual, func_name
            )),
            line: loc.line,
            col: loc.col,
        }
    }
    fn err_binop(&self, loc: &Loc, op: &Operator, t0: Type, t1: Type) -> CompileError {
        CompileError {
            e_type: ErrorType::TypeError(format!(
                "Cannot use binary operator '{:#?}' on values of type '{:#?}' and '{:#?}'",
                op, t0, t1
            )),
            line: loc.line,
            col: loc.col,
        }
    }
    fn add_err(&mut self, error: CompileError) {
        for err in self.errors.iter() {
            if error.e_type == err.e_type && error.line == err.line && error.col == err.col {
                return;
            }
        }
        self.errors.push(error);
    }
}
