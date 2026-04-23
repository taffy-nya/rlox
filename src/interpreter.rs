use crate::{
    error::EvalError, 
    stmt::Stmt, 
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
            if let Err(err) = stmt.exec(&self.env) {
                errors.push(err);
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
