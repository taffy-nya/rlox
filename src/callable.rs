use crate::{
    error::EvalError, 
    stmt::Stmt, 
    token::Literal,
    env::Env,
};

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function(Function),
    Native(NativeFunction),
}

impl Callable {
    pub fn arity(&self) -> usize {
        match self {
            Callable::Function(func) => func.arity(),
            Callable::Native(native) => native.arity(),
        }
    }
    pub fn call(&self, args: Vec<Literal>) -> Result<Literal, EvalError> {
        match self {
            Callable::Function(func) => func.call(args),
            Callable::Native(native) => native.call(args),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Callable::Function(func) => &func.name,
            Callable::Native(native) => &native.name,
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub env: Env,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("params", &self.params)
            .field("body", &self.body)
            .field("env", &"<closure>")
            .finish()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.params == other.params
            && self.body == other.body
    }
}

impl Function {
    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn call(&self, args: Vec<Literal>) -> Result<Literal, EvalError> {
        let env = Env::enclosed(self.env.clone());
        for (param, arg) in self.params.iter().zip(args) {
            env.define(param.clone(), arg);
        }

        for stmt in &self.body {
            stmt.exec(&env)?;
        }
        Ok(Literal::Nil)
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(Vec<Literal>) -> Result<Literal, EvalError>,
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

impl NativeFunction {
    pub fn arity(&self) -> usize {
        self.arity
    }
    pub fn call(&self, args: Vec<Literal>) -> Result<Literal, EvalError> {
        (self.func)(args)
    }
}

pub fn native_functions() -> Vec<(&'static str, Callable)> {
    vec![
        ("clock", Callable::Native(NativeFunction {
            name: "clock".to_string(),
            arity: 0,
            func: |_| Ok(Literal::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64())),
        })),
    ]
}