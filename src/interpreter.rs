use crate::stmt::Stmt;
use crate::error::EvalError;

use std::collections::HashMap;
use crate::token::Literal;

pub struct Environment {
    scopes: Vec<HashMap<String, Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn new_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Literal, EvalError> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        Err(EvalError)
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<(), EvalError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(EvalError)
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
