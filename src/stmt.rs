use crate::{
    callable::{Callable, Function}, env::Env, error::EvalError, expr::Expr, token::{Literal, Token}
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
    Function { name: Token, params: Vec<Token>, body: Vec<Stmt> },
    Return { keyword: Token, value: Expr },
}

pub enum ExecFlow {
    Normal,
    Return { keyword: Token, value: Literal },
}

impl Stmt {
    pub fn exec(&self, env: &Env) -> Result<ExecFlow, EvalError> {
        match self {
            Stmt::Expression { expression } => {
                expression.eval(env)?;
                Ok(ExecFlow::Normal)
            },
            Stmt::Print { expression } => {
                let val = expression.eval(env)?;
                println!("{}", val);
                Ok(ExecFlow::Normal)
            },
            Stmt::Var { name, initializer } => {
                let val = initializer.eval(env)?;
                env.define(name.lexeme.clone(), val);
                Ok(ExecFlow::Normal)
            }
            Stmt::Block { statements } => {
                let block_env = Env::enclosed(env.clone());
                for stmt in statements {
                    match stmt.exec(&block_env)? {
                        ExecFlow::Normal => continue,
                        flow @ ExecFlow::Return { .. } => return Ok(flow),
                    }
                }
                Ok(ExecFlow::Normal)
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = condition.eval(env)?;
                if cond_val.is_truthy() {
                    then_branch.exec(env)
                } else if let Some(else_stmt) = else_branch {
                    else_stmt.exec(env)
                } else {
                    Ok(ExecFlow::Normal)
                }
            }
            Stmt::While { condition, body } => {
                while condition.eval(env)?.is_truthy() {
                    match body.exec(env)? {
                        ExecFlow::Normal => continue,
                        flow @ ExecFlow::Return { .. } => return Ok(flow),
                    }
                }
                Ok(ExecFlow::Normal)
            }
            Stmt::Function { name, params, body } => {
                let func = Function {
                    name: name.lexeme.clone(),
                    params: params.iter().map(|t| t.lexeme.clone()).collect(),
                    body: body.clone(),
                    env: env.clone(),
                };
                env.define(name.lexeme.clone(), Literal::Callable(Rc::new(Callable::Function(func))));
                Ok(ExecFlow::Normal)
            }
            Stmt::Return { keyword, value } => {
                Ok(ExecFlow::Return { keyword: keyword.clone(), value: value.eval(env)? })
            }
        }
    }
}