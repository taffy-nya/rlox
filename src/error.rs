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

pub fn report(line: usize, message: &str) {
    eprintln!("Error on line {}: {}", line, message);
    set_had_error();
}

pub fn report_at(line: usize, at: &str, message: &str) {
    eprintln!("Error on line {} {}: {}", line, at, message);
    set_had_error();
}

pub fn report_token(token: &Token, message: &str) {
    match token.token_type {
        TokenType::Eof => report_at(token.line, "at end", message),
        _ => report_at(token.line, &format!("at '{}'", token.lexeme), message),
    }
    set_had_error();
}
