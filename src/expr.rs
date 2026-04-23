use crate::{
    error::EvalError, 
    token::{Literal, Token, TokenType},
    env::Env,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
    Assign { name: Token, value: Box<Expr> },
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Call { callee: Box<Expr>, rparen: Token, args: Vec<Expr> },
}

impl Expr {
    fn eval_error(&self, token: &Token, message: &str) -> Result<Literal, EvalError> {
        Err(EvalError::new(token, message))
    }

    pub fn eval(&self, env: &Env) -> Result<Literal, EvalError> {
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
                        Ok(val) => Ok(Literal::Bool(!val.is_truthy())),
                        _ => self.eval_error(operator, "Operand must be a boolean.")
                    }
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
            Expr::Logical { left, operator, right } => {
                let left_val = left.eval(env)?;
                match operator.token_type {
                    TokenType::Or if left_val.is_truthy() => Ok(left_val),
                    TokenType::And if !left_val.is_truthy() => Ok(left_val),
                    TokenType::Or | TokenType::And => right.eval(env),
                    _ => self.eval_error(operator, "Unknown logical operator.")
                }
            }
            Expr::Call { callee, rparen, args } => {
                let callee_val = callee.eval(env)?;
                let arg_vals = args.iter().map(|arg| arg.eval(env)).collect::<Result<Vec<_>, _>>()?;
                match callee_val {
                    Literal::Callable(callable) if arg_vals.len() == callable.arity() => callable.call(arg_vals),
                    Literal::Callable(callable) => self.eval_error(
                        rparen, &format!("Expected {} arguments but got {}.", 
                        callable.arity(), arg_vals.len())),
                    _ => self.eval_error(rparen, "Can only call functions and classes."),
                }
            }
        }
    }
}
