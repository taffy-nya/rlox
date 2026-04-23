use crate::{
    error::ParseError, 
    expr::Expr, 
    token::{Token, TokenType, Literal},
    stmt::Stmt,
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    loop_depth: usize,
    function_depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0, loop_depth: 0, function_depth: 0 }
    }

    // program -> declaration* EOF
    pub fn parse(mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(error) => {
                    errors.push(error);
                    self.synchronize();
                }
            }
        }
        if errors.is_empty() {
            Ok(stmts)
        } else {
            Err(errors)
        }
    }

    pub fn parse_as_expr(mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            return self.parse_error("Expect end after expression.");
        }
        Ok(expr)
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
        !self.is_at_end() && self.peek().token_type == *token_type
    }

    fn check_types(&self, types: &[TokenType]) -> bool {
        !self.is_at_end() && types.contains(&self.peek().token_type)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check_type(&token_type) {
            Ok(self.advance())
        } else {
            self.parse_error(msg)
        }
    }
    
    fn match_type(&mut self, types: &[TokenType]) -> Option<Token> {
        if self.check_types(types) {
            Some(self.advance().clone())
        } else {
            None
        }
    }
    
    fn parse_error<T>(&self, message: &str) -> Result<T, ParseError> {
        Err(ParseError::new(self.peek(), message))
    }

    fn parse_error_at<T>(&self, token: &Token, message: &str) -> Result<T, ParseError> {
        Err(ParseError::new(token, message))
    }
    
    /// declaration -> varDecl | funcDecl | statement
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_type(&[TokenType::Var]).is_some() {
            self.var_declaration()
        } else if self.match_type(&[TokenType::Fun]).is_some() {
            self.func_declaration()
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

    /// funcDecl -> "fun" function
    fn func_declaration(&mut self) -> Result<Stmt, ParseError> {
        self.function("function")
    }

    /// function -> IDENTIFIER "(" parameters? ")" block
    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {kind} name."))?.clone();
        self.consume(TokenType::LeftParen, &format!("Expect '(' after {kind} name."))?;
        let mut params = Vec::new();
        if !self.check_type(&TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    self.parse_error_at(self.peek(), "Can't have more than 255 parameters.")?;
                }
                params.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?.clone()
                );
                if self.match_type(&[TokenType::Comma]).is_none() {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, &format!("Expect '{{' before {kind} body."))?;
        self.function_depth += 1;
        let body = match self.block()? {
            Stmt::Block { statements } => statements,
            _ => unreachable!(),
        };
        self.function_depth -= 1;
        Ok(Stmt::Function { name, params, body })
    }

    /// statement -> exprstmt | ifstmt | whilestmt | forstmt | returnstmt | continuestmt | breakstmt | printstmt | block
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_type(&[TokenType::If]).is_some() {
            self.if_statement()
        } else if self.match_type(&[TokenType::While]).is_some() {
            self.while_statement()
        } else if self.match_type(&[TokenType::For]).is_some() {
            self.for_statement()
        } else if self.match_type(&[TokenType::Return]).is_some() {
            self.return_statement()
        } else if self.match_type(&[TokenType::Continue]).is_some() {
            self.continue_statement()
        } else if self.match_type(&[TokenType::Break]).is_some() {
            self.break_statement()
        } else if self.match_type(&[TokenType::Print]).is_some() {
            self.print_statement()
        } else if self.match_type(&[TokenType::LeftBrace]).is_some() {
            self.block()
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

    /// ifstmt -> "if" "("? expression ")"? statement ( "else" statement )?
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.match_type(&[TokenType::LeftParen]);
        let condition = self.expression()?;
        self.match_type(&[TokenType::RightParen]);
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_type(&[TokenType::Else]).is_some() {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    /// whilestmt -> "while" "("? expression ")"? statement
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.match_type(&[TokenType::LeftParen]);
        let condition = self.expression()?;
        self.match_type(&[TokenType::RightParen]);
        self.loop_depth += 1;
        let body = Box::new(self.statement()?);
        self.loop_depth -= 1;
        Ok(Stmt::While { condition, body })
    }

    /// forstmt -> "for" "(" ( varDecl | exprstmt | ";" ) expression? ";" expression? ")" statement
    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.match_type(&[TokenType::Semicolon]).is_some() {
            None
        } else if self.match_type(&[TokenType::Var]).is_some() {
            Some(Box::new(self.var_declaration()?))
        } else {
            Some(Box::new(self.expression_statement()?))
        };
        let condition = if !self.check_type(&TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal { value: Literal::Bool(true) }
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
        let increment = if !self.check_type(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        self.loop_depth += 1;
        let body = Box::new(self.statement()?);
        self.loop_depth -= 1;
        Ok(Stmt::For { initializer, condition, increment, body })
    }

    /// printstmt -> "print" expression ";"
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: expr })
    }

    /// returnstmt -> "return" expression? ";"
    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        if self.function_depth == 0 {
            return self.parse_error_at(&keyword, "Can't return from top-level code.");
        }
        let value = if !self.check_type(&TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal { value: Literal::Nil }
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    /// continuestmt -> "continue" ";"
    fn continue_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        if self.loop_depth == 0 {
            return self.parse_error_at(&keyword, "Can't use 'continue' outside of a loop.");
        }
        self.consume(TokenType::Semicolon, "Expect ';' after 'continue'.")?;
        Ok(Stmt::Continue { keyword })
    }

    /// breakstmt -> "break" ";"
    fn break_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        if self.loop_depth == 0 {
            return self.parse_error_at(&keyword, "Can't use 'break' outside of a loop.");
        }
        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
        Ok(Stmt::Break { keyword })
    }

    /// block -> "{" declaration* "}"
    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut stmts = Vec::new();
        while !self.check_type(&TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block { statements: stmts })
    }

    /// expression -> assignment
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    /// assignment -> IDENTIFIER "=" assignment | logic_or
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logic_or()?;
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

    /// logic_or -> logic_and ( "or" logic_and )*
    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logic_and()?;
        while let Some(op) = self.match_type(&[TokenType::Or]) {
            let right = self.logic_and()?;
            expr = Expr::Logical { left: Box::new(expr), operator: op, right: Box::new(right) };
        }
        Ok(expr)
    }

    /// logic_and -> equality ( "and" equality )*
    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while let Some(op) = self.match_type(&[TokenType::And]) {
            let right = self.equality()?;
            expr = Expr::Logical { left: Box::new(expr), operator: op, right: Box::new(right) };
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

    /// unary -> ( "!" | "-" ) unary | call
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(operator) = self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.call()
    }

    /// call -> primary ( "(" arguments? ")" )* ;
    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_type(&[TokenType::LeftParen]).is_some() {
                expr = self.arguments(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// arguments -> expression ( "," expression )*
    fn arguments(&mut self, expr: Expr) -> Result<Expr, ParseError> {
        let mut args = Vec::new();
        if !self.check_type(&TokenType::RightParen) {
            loop {
                args.push(self.expression()?);
                if self.match_type(&[TokenType::Comma]).is_none() {
                    break;
                }
                if args.len() > 255 {
                    self.parse_error_at(self.peek(), "Can't have more than 255 arguments.")?;
                }
            }
        }
        let rparen = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?.clone();
        Ok(Expr::Call { callee: Box::new(expr), rparen, args })
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

            if self.check_types(&[
                TokenType::Class,
                TokenType::Fun,
                TokenType::Var,
                TokenType::For,
                TokenType::If,
                TokenType::While,
                TokenType::Print,
                TokenType::Return,
            ]) {
                return;
            }

            self.advance();
        }
    }
} 
