use crate::expr::Expr;
use crate::error::EvalError;
use crate::token::Token;
use crate::interpreter::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
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
                env.define(&name.lexeme, val);
                Ok(())
            }
        }
    }
}