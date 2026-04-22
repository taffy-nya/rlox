use std::env;
use std::io::Write;

mod error;
mod token;
mod expr;
mod parser;
mod stmt;
mod interpreter;

fn run(code: &str, interpreter: &mut interpreter::Interpreter) -> bool {
    let scanner = token::Scanner::new(code);

    let tokens = match scanner.tokenize() {
        Ok(tokens) => tokens,
        Err(errors) => {
            for error in errors {
                eprintln!("{error}");
            }
            return false;
        }
    };

    let parser = parser::Parser::new(&tokens);

    let stmts = match parser.parse() {
        Ok(stmts) => stmts,
        Err(errors) => {
            for error in errors {
                eprintln!("{error}");
            }
            return false;
        }
    };

    match interpreter.work(&stmts) {
        Ok(_) => true,
        Err(errors) => {
            for error in errors {
                eprintln!("{error}");
            }
            false
        }
    }
}

fn run_file(path: &String) {
    let code = std::fs::read_to_string(path).expect("Failed to read file");
    let mut interpreter = interpreter::Interpreter::new();

    if !run(code.as_str(), &mut interpreter) {
        std::process::exit(1);
    }
}

fn update_prompt_state(indent: &mut usize, in_string: &mut bool, line: &str) {
    let mut delta = 0;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if *in_string {
            if c == '"' { *in_string = false; }
            continue;
        }
        match c {
            '"' => *in_string = true,
            '/' if chars.peek() == Some(&'/') => break,
            '{' => delta += 1,
            '}' => delta -= 1,
            _ => {}
        }
    }

    if delta < 0 {
        *indent = indent.saturating_sub((-delta) as usize);
    } else {
        *indent += delta as usize;
    }
}

fn run_prompt() {
    let mut code = String::new();
    let mut interpreter = interpreter::Interpreter::new();
    let mut indent = 0;
    let mut in_string = false;
    const INDENT_STR: &str = "  ";
    loop {
        print!("{}", if code.is_empty() { "> " } else { "| " });
        print!("{}", INDENT_STR.repeat(indent));
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).expect("Failed to read line") == 0 {
            break;
        }
        let line = line.trim_end_matches(['\r', '\n']);
        let continues = line.ends_with('\\');
        let line = if continues { line.trim_end_matches('\\') } else { line };

        code.push_str(line);
        code.push('\n');
        update_prompt_state(&mut indent, &mut in_string, line);

        if continues || indent > 0 || in_string { continue; }

        if !code.trim().is_empty() {
            run(code.as_str(), &mut interpreter);
        }

        code.clear();
        indent = 0;
        in_string = false;
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        eprintln!("Usage: {} [script]", args[0]);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
