use crate::expr::Expr;
use crate::error::EvalError;
use crate::token::Token;
use crate::interpreter::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
}

impl Stmt {
    pub fn exec(&self, env: &mut Environment) -> Result<(), EvalError> {
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
                env.new_scope();
                let res = statements.iter().try_for_each(|stmt| stmt.exec(env));
                env.end_scope();
                res
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
        }
    }
}