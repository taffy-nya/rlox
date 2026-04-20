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
    pub fn eval(&self) -> Literal {
        match self {
            Expr::Binary { left, operator, right } => {
                let left_val = left.eval();
                let right_val = right.eval();
                match operator.token_type {
                    TokenType::Plus => match (left_val, right_val) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                        (Literal::String(l), Literal::String(r)) => Literal::String(l + &r),
                        _ => {
                            error::report_token(operator, "Operands must be two numbers or two strings.");
                            Literal::Nil
                        }
                    },
                    TokenType::Minus => match (left_val, right_val) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l - r),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Literal::Nil
                        }
                    },
                    TokenType::Star => match (left_val, right_val) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l * r),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Literal::Nil
                        }
                    },
                    TokenType::Slash => match (left_val, right_val) {
                        (Literal::Number(l), Literal::Number(r)) => Literal::Number(l / r),
                        _ => {
                            error::report_token(operator, "Operands must be numbers.");
                            Literal::Nil
                        }
                    },
                    _ => {
                        error::report_token(operator, "Unknown operator.");
                        Literal::Nil
                    }
                }
            }
            Expr::Unary { operator, right } => {
                let right_val = right.eval();
                match operator.token_type {
                    TokenType::Minus => match right_val {
                        Literal::Number(n) => Literal::Number(-n),
                        _ => {
                            error::report_token(operator, "Operand must be a number.");
                            Literal::Nil
                        }
                    },
                    TokenType::Bang => match right_val {
                        Literal::Bool(b) => Literal::Bool(!b),
                        _ => {
                            error::report_token(operator, "Operand must be a boolean.");
                            Literal::Nil
                        }
                    },
                    _ => {
                        error::report_token(operator, "Unknown operator.");
                        Literal::Nil
                    }
                }
            }
            Expr::Grouping { expression } => expression.eval(),
            Expr::Literal { value } => value.clone(),
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
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

    fn check_type(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().token_type == *token_type
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check_type(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    /// expression -> equality
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
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

        while self.match_type(&[
            TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
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

        while self.match_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
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

        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        expr
    }

    /// unary -> ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Expr {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary { operator, right: Box::new(right) };
        }
        self.primary()
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Expr {
        if self.match_type(&[TokenType::False]) {
            return Expr::Literal { value: Literal::Bool(false) };
        }
        if self.match_type(&[TokenType::True]) {
            return Expr::Literal { value: Literal::Bool(true) };
        }
        if self.match_type(&[TokenType::Nil]) {
            return Expr::Literal { value: Literal::Nil };
        }
        if self.match_type(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal { value: self.previous().literal.clone().unwrap() };
        }
        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping { expression: Box::new(expr) };
        } else {
            error::report_token(self.peek(), "Expect expression.");
            Expr::Literal { value: Literal::Nil }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check_type(&token_type) {
            self.advance();
            return;
        }
        error::report_token(self.peek(), message);
    }

    // fn synchronize(&mut self) {
    //     unimplemented!();
    // }

} 