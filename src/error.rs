use std::sync::atomic::{AtomicBool, Ordering};

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

pub fn error(line: usize, message: &str) {
    eprintln!("Error on line {}: {}", line, message);
    set_had_error();
}
