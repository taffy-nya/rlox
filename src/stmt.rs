use crate::{
    error::EvalError,
    expr::Expr,
    token::{Token, Literal},
    callable::{Callable, Function},
    env::Env,
};

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { condition: Expr, body: Box<Stmt> },
    Function { name: Token, params: Vec<Token>, body: Vec<Stmt> }
}

impl Stmt {
    pub fn exec(&self, env: &Env) -> Result<(), EvalError> {
        match self {
            Stmt::Expression { expression } => {
                expression.eval(env)?;
                Ok(())
            },
            Stmt::Print { expression } => {
                let val = expression.eval(env)?;
                println!("{}", val);
                Ok(())
            },
            Stmt::Var { name, initializer } => {
                let val = initializer.eval(env)?;
                env.define(name.lexeme.clone(), val);
                Ok(())
            }
            Stmt::Block { statements } => {
                let block_env = Env::enclosed(env.clone());
                statements.iter().try_for_each(|stmt| stmt.exec(&block_env))
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = condition.eval(env)?;
                if cond_val.is_truthy() {
                    then_branch.exec(env)
                } else if let Some(else_stmt) = else_branch {
                    else_stmt.exec(env)
                } else {
                    Ok(())
                }
            }
            Stmt::While { condition, body } => {
                while condition.eval(env)?.is_truthy() {
                    body.exec(env)?;
                }
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let func = Function {
                    name: name.lexeme.clone(),
                    params: params.iter().map(|t| t.lexeme.clone()).collect(),
                    body: body.clone(),
                    env: env.clone(),
                };
                env.define(name.lexeme.clone(), Literal::Callable(Rc::new(Callable::Function(func))));
                Ok(())
            }
        }
    }
}