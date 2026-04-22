use crate::error;
use crate::error::ParseError;
use crate::expr::Expr;
use crate::token::{Token, TokenType, Literal};
use crate::stmt::Stmt;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    // program -> declaration* EOF
    pub fn parse(mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(_) => self.synchronize(),
            }
        }
        stmts
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

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.peek().token_type == token_type {
            Ok(self.advance())
        } else {
            self.parse_error(msg)
        }
    }
    
    fn match_type(&mut self, types: &[TokenType]) -> Option<Token> {
        if types.contains(&self.peek().token_type) {
            Some(self.advance().clone())
        } else {
            None
        }
    }
    
    fn parse_error<T>(&self, message: &str) -> Result<T, ParseError> {
        error::report_token("Parse error", self.peek(), message);
        Err(ParseError)
    }

    fn parse_error_at(&self, token: &Token, message: &str) -> Result<(), ParseError> {
        error::report_token("Parse error", token, message);
        Err(ParseError)
    }
    
    /// declaration -> varDecl | statement
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_type(&[TokenType::Var]).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        }        
    }

    /// varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?.clone();
        let initializer = if self.match_type(&[TokenType::Equal]).is_some() {
            self.expression()?
        } else {
            Expr::Literal { value: Literal::Nil }
        }; 
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, initializer })
    }

    /// statement -> exprstmt | printstmt
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_type(&[TokenType::Print]).is_some() {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    /// exprstmt -> expression ";"
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    /// printstmt -> "print" expression ";"
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: expr })
    }

    /// expression -> assignment
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    /// assignment -> IDENTIFIER "=" assignment | equality
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;
        if self.match_type(&[TokenType::Equal]).is_some() {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign { name, value: Box::new(value) });
            }
            self.parse_error_at(&equals, "Invalid assignment target.")?;
        }
        Ok(expr)
    }
    
    /// equality -> comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while let Some(operator) = self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        Ok(expr)
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while let Some(operator) = self.match_type(&[
            TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual,
        ]) {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        Ok(expr)
    }

    /// term -> factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while let Some(operator) = self.match_type(&[TokenType::Minus, TokenType::Plus]) {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        Ok(expr)
    }

    /// factor -> unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while let Some(operator) = self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr), operator, right: Box::new(right)
            };
        }
        Ok(expr)
    }

    /// unary -> ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(operator) = self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.primary()
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(&[TokenType::False]).is_some() {
            return Ok(Expr::Literal { value: Literal::Bool(false) });
        }
        if self.match_type(&[TokenType::True]).is_some() {
            return Ok(Expr::Literal { value: Literal::Bool(true) });
        }
        if self.match_type(&[TokenType::Nil]).is_some() {
            return Ok(Expr::Literal { value: Literal::Nil });
        }
        if self.match_type(&[TokenType::Number, TokenType::String]).is_some() {
            return Ok(Expr::Literal { value: self.previous().literal.clone().unwrap() });
        }
        if self.match_type(&[TokenType::LeftParen]).is_some() {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }
        if self.match_type(&[TokenType::Identifier]).is_some() {
            return Ok(Expr::Variable { name: self.previous().clone() });
        }
        self.parse_error("Expect expression.")
    }


    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }

            self.advance();
        }
    }
} 
