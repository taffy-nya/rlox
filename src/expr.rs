use crate::token::{Literal, TokenType, Token};
use crate::error::EvalError;
use crate::interpreter::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
    Assign { name: Token, value: Box<Expr> },
}

impl Expr {
    fn eval_error(&self, token: &Token, message: &str) -> Result<Literal, EvalError> {
        Err(EvalError::new(token, message))
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Literal, EvalError> {
        match self {
            Expr::Binary { left, operator, right } => {
                let left_val = left.eval(env);
                let right_val = right.eval(env);
                match operator.token_type {
                    TokenType::Plus => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l + r)),
                        (Ok(Literal::String(l)), Ok(Literal::String(r))) => Ok(Literal::String(l + &r)),
                        _ => self.eval_error(operator, "Operands must be two numbers or two strings.")
                    },
                    TokenType::Minus => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l - r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    TokenType::Star => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l * r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    TokenType::Slash => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l / r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    TokenType::EqualEqual => match (left_val, right_val) {
                        (Ok(l), Ok(r)) => Ok(Literal::Bool(l == r)),
                        _ => self.eval_error(operator, "Operands must be of the same type.")
                    },
                    TokenType::BangEqual => match (left_val, right_val) {
                        (Ok(l), Ok(r)) => Ok(Literal::Bool(l != r)),
                        _ => self.eval_error(operator, "Operands must be of the same type.")
                    },
                    TokenType::Greater => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l > r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    TokenType::GreaterEqual => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l >= r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    TokenType::Less => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l < r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    TokenType::LessEqual => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l <= r)),
                        _ => self.eval_error(operator, "Operands must be numbers.")
                    },
                    _ => self.eval_error(operator, "Unknown operator.")
                }
            }
            Expr::Unary { operator, right } => {
                let right_val = right.eval(env);
                match operator.token_type {
                    TokenType::Minus => match right_val {
                        Ok(Literal::Number(n)) => Ok(Literal::Number(-n)),
                        _ => self.eval_error(operator, "Operand must be a number.")
                    },
                    TokenType::Bang => match right_val {
                        Ok(Literal::Bool(b)) => Ok(Literal::Bool(!b)),
                        Ok(Literal::Nil) => Ok(Literal::Bool(true)),
                        _ => Ok(Literal::Bool(false))
                    },
                    _ => self.eval_error(operator, "Unknown operator.")
                }
            }
            Expr::Grouping { expression } => expression.eval(env),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Variable { name } => match env.get(&name.lexeme) {
                Some(val) => Ok(val.clone()),
                None => self.eval_error(name, "Undefined variable.")
            }
            Expr::Assign { name, value } => {
                let value = value.eval(env)?;
                match env.assign(&name.lexeme, value.clone()) {
                    Some(_) => Ok(value),
                    None => self.eval_error(name, "Undefined variable.")
                }
            }
        }
    }
}
