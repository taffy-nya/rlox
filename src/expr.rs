use crate::token::{Literal, TokenType, Token};
use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => format!("({} {} {})", operator.lexeme, left.print(), right.print()),
            Expr::Grouping { expression } => format!("(group {})", expression.print()),
            Expr::Literal { value } => match value {
                Literal::Number(n) => n.to_string(),
                Literal::String(s) => s.clone(),
                Literal::Bool(b) => b.to_string(),
                Literal::Nil => "nil".to_string(),
            },
            Expr::Unary { operator, right } => format!("({} {})", operator.lexeme, right.print()),
        }
    }
    pub fn eval(&self) -> Result<Literal, String> {
        match self {
            Expr::Binary { left, operator, right } => {
                let left_val = left.eval();
                let right_val = right.eval();
                match operator.token_type {
                    TokenType::Plus => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l + r)),
                        (Ok(Literal::String(l)), Ok(Literal::String(r))) => Ok(Literal::String(l + &r)),
                        _ => {
                            error::report_token(operator, "Operands must be two numbers or two strings.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::Minus => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l - r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::Star => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l * r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::Slash => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Number(l / r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::EqualEqual => match (left_val, right_val) {
                        (Ok(l), Ok(r)) => Ok(Literal::Bool(l == r)),
                        _ => {
                            error::report_token(operator, "Operands must be of the same type.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::BangEqual => match (left_val, right_val) {
                        (Ok(l), Ok(r)) => Ok(Literal::Bool(l != r)),
                        _ => {
                            error::report_token(operator, "Operands must be of the same type.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::Greater => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l > r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::GreaterEqual => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l >= r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::Less => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l < r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    TokenType::LessEqual => match (left_val, right_val) {
                        (Ok(Literal::Number(l)), Ok(Literal::Number(r))) => Ok(Literal::Bool(l <= r)),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Err("Invalid operand types".into())
                        }
                    },
                    _ => {
                        error::report_token(operator, "Unknown operator.");
                        Err("Unknown operator".into())
                    }
                }
            }
            Expr::Unary { operator, right } => {
                let right_val = right.eval();
                match operator.token_type {
                    TokenType::Minus => match right_val {
                        Ok(Literal::Number(n)) => Ok(Literal::Number(-n)),
                        _ => {
                            error::report_token(operator, "Operand must be a number.");
                            Err("Invalid operand type".into())
                        }
                    },
                    TokenType::Bang => match right_val {
                        Ok(Literal::Bool(b)) => Ok(Literal::Bool(!b)),
                        Ok(Literal::Nil) => Ok(Literal::Bool(true)),
                        _ => Ok(Literal::Bool(false))
                    },
                    _ => {
                        error::report_token(operator, "Unknown operator.");
                        Err("Unknown operator".into())
                    }
                }
            }
            Expr::Grouping { expression } => expression.eval(),
            Expr::Literal { value } => Ok(value.clone()),
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Expr {
        self.expression()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn match_type(&mut self, types: &[TokenType]) -> Option<Token> {
        if types.contains(&self.peek().token_type) {
            return Some(self.advance().clone());
        } else {
            None
        }
    }
    
    /// expression -> equality
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        
        while let Some(operator) = self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        expr
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(operator) = self.match_type(&[
            TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual,
        ]) {
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        expr
    }

    /// term -> factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(operator) = self.match_type(&[TokenType::Minus, TokenType::Plus]) {
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        expr
    }

    /// factor -> unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(operator) = self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        expr
    }

    /// unary -> ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Expr {
        if let Some(operator) = self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary();
            return Expr::Unary { operator, right: Box::new(right) };
        }
        self.primary()
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Expr {
        if self.match_type(&[TokenType::False]).is_some() {
            return Expr::Literal { value: Literal::Bool(false) };
        }
        if self.match_type(&[TokenType::True]).is_some() {
            return Expr::Literal { value: Literal::Bool(true) };
        }
        if self.match_type(&[TokenType::Nil]).is_some() {
            return Expr::Literal { value: Literal::Nil };
        }
        if self.match_type(&[TokenType::Number, TokenType::String]).is_some() {
            return Expr::Literal { value: self.previous().literal.clone().unwrap() };
        }
        if self.match_type(&[TokenType::LeftParen]).is_some() {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping { expression: Box::new(expr) };
        } else {
            error::report_token(self.peek(), "Expect expression.");
            Expr::Literal { value: Literal::Nil }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.match_type(&[token_type]).is_some() {
            return;
        }
        error::report_token(self.peek(), message);
    }

    // fn synchronize(&mut self) {
    //     unimplemented!();
    // }

} 

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::error;
    use crate::token::Scanner;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn parse(source: &str) -> Expr {
        let tokens = Scanner::new(source).tokenize();
        Parser::new(&tokens).parse()
    }

    fn eval(source: &str) -> Result<Literal, String> {
        parse(source).eval()
    }

    #[test]
    fn evaluates_arithmetic_precedence() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        assert_eq!(eval("1 + 2 * 3"), Ok(Literal::Number(7.0)));
        assert!(!error::had_error());
    }

    #[test]
    fn evaluates_grouping() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        assert_eq!(eval("(1 + 2) * 3"), Ok(Literal::Number(9.0)));
        assert!(!error::had_error());
    }

    #[test]
    fn evaluates_equality_and_comparison() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        assert_eq!(eval("1 + 2 * 3 == 7"), Ok(Literal::Bool(true)));
        assert_eq!(eval("3 < 2 == false"), Ok(Literal::Bool(true)));
        assert_eq!(eval("nil == nil"), Ok(Literal::Bool(true)));
        assert!(!error::had_error());
    }

    #[test]
    fn evaluates_truthiness() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        assert_eq!(eval("!false"), Ok(Literal::Bool(true)));
        assert_eq!(eval("!nil"), Ok(Literal::Bool(true)));
        assert_eq!(eval("!123"), Ok(Literal::Bool(false)));
        assert!(!error::had_error());
    }

    #[test]
    fn reports_runtime_type_error() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        assert!(eval("1 + \"x\"").is_err());
        assert!(error::had_error());
    }

    #[test]
    fn reports_parse_error_for_missing_operand() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        let expr = parse("1 +");

        assert_eq!(expr.print(), "(+ 1 nil)");
        assert!(error::had_error());
    }

    #[test]
    fn reports_parse_error_for_missing_right_paren() {
        let _guard = TEST_LOCK.lock().unwrap();
        error::reset_had_error();

        let expr = parse("(1 + 2");

        assert_eq!(expr.print(), "(group (+ 1 2))");
        assert!(error::had_error());
    }
}
