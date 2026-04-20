use std::sync::atomic::{AtomicBool, Ordering};
use crate::token::{Token, TokenType};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn set_had_error() {
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub fn reset_had_error() {
    HAD_ERROR.store(false, Ordering::Relaxed);
}

pub fn had_error() -> bool { 
    HAD_ERROR.load(Ordering::Relaxed)
}

pub fn report(error: &str, line: usize, message: &str) {
    eprintln!("{} on line {}: {}", error, line, message);
    set_had_error();
}

pub fn report_at(error: &str, line: usize, at: &str, message: &str) {
    eprintln!("{} on line {} {}: {}", error, line, at, message);
    set_had_error();
}

pub fn report_token(error: &str, token: &Token, message: &str) {
    match token.token_type {
        TokenType::Eof => report_at(error, token.line, "at end", message),
        _ => report_at(error, token.line, &format!("at '{}'", token.lexeme), message),
    }
    set_had_error();
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvalError;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError;
