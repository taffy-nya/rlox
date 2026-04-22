use crate::stmt::Stmt;
use crate::error::EvalError;

use std::collections::HashMap;
use crate::token::Literal;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<Literal, EvalError> {
        self.values.get(name).cloned().ok_or(EvalError)
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<(), EvalError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(EvalError)
        }
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }
    pub fn work(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            if let Err(_) = stmt.exec(&mut self.env) {
                // Handle error (e.g., print it)
            }
        }
    }
}