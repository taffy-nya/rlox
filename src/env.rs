use crate::{
    callable::{Callable, native_functions},
    token::Literal,
};

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Env>,
}

#[derive(Clone)]
pub struct Env(Rc<RefCell<Environment>>);

impl Env {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: None,
        })))
    }

    pub fn enclosed(enclosing: Env) -> Self {
        Self(Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        })))
    }

    pub fn global() -> Self {
        let env = Env::new();
        for (name, function) in native_functions() {
            env.define_native(name, function);
        }
        env
    }
    
    pub fn define(&self, name: String, value: Literal) {
        self.0.borrow_mut().values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Literal> {
        if let Some(value) = self.0.borrow().values.get(name) {
            return Some(value.clone());
        }

        let enclosing = self.0.borrow().enclosing.clone();
        enclosing.and_then(|env| env.get(name))
    }

    pub fn assign(&self, name: &str, value: Literal) -> Option<()> {
        if self.0.borrow().values.contains_key(name) {
            self.0.borrow_mut().values.insert(name.to_string(), value);
            return Some(());
        }

        let enclosing = self.0.borrow().enclosing.clone();
        enclosing.and_then(|env| env.assign(name, value))
    }

    pub fn define_native(&self, name: &'static str, function: Callable) {
        self.define(name.to_string(), Literal::Callable(Rc::new(function)));
    }
}

