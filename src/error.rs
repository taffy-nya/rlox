use std::fmt;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct EvalError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub struct SyntaxError {
    pub line: usize,
    pub column: usize,
    pub lexeme: String,
    pub message: String,
}

impl EvalError {
    pub fn new(token: &Token, message: &str) -> Self {
        Self {
            token: token.clone(),
            message: message.to_string(),
        }
    }
}

impl ParseError {
    pub fn new(token: &Token, message: &str) -> Self {
        Self {
            token: token.clone(),
            message: message.to_string(),
        }
    }
}

impl SyntaxError {
    pub fn new(line: usize, column: usize, lexeme: String, message: &str) -> Self {
        Self {
            line,
            column,
            lexeme,
            message: message.to_string(),
        }
    }
}

fn format_token_location(token: &Token) -> String {
    match token.token_type {
        TokenType::Eof => format!("line {}:{} at end", token.line, token.column),
        _ => format!("line {}:{} at '{}'", token.line, token.column, token.lexeme),
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error on {}: {}",
            format_token_location(&self.token),
            self.message
        )
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Evaluation error on {}: {}",
            format_token_location(&self.token),
            self.message
        )
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lexeme.is_empty() {
            write!(
                f,
                "Syntax error on line {}:{}: {}",
                self.line, self.column, self.message
            )
        } else {
            write!(
                f,
                "Syntax error on line {}:{} at '{}': {}",
                self.line, self.column, self.lexeme, self.message
            )
        }
    }
}
