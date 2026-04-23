use crate::{
    error::EvalError, 
    stmt::{Stmt, ExecFlow}, 
    expr::Expr, 
    token::Literal,
    env::Env,
};

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::global(),
        }
    }

    pub fn work(&mut self, stmts: &[Stmt]) -> Result<(), Vec<EvalError>> {
        let mut errors = Vec::new();
        for stmt in stmts {
            match stmt.exec(&self.env) {
                Ok(ExecFlow::Normal) => continue,
                Ok(ExecFlow::Return { keyword, .. }) => {
                    errors.push(EvalError {
                        token: keyword,
                        message: "Cannot return from top-level code.".to_string(),
                    });
                }
                Ok(ExecFlow::Break { keyword }) => {
                    errors.push(EvalError::new(&keyword, "Can't use 'break' outside of a loop."));
                }
                Ok(ExecFlow::Continue { keyword }) => {
                    errors.push(EvalError::new(&keyword, "Can't use 'continue' outside of a loop."));
                }
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn eval(&self, expr: &Expr) -> Result<Literal, EvalError> {
        expr.eval(&self.env)
    }
}
