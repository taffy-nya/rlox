use std::env;
use std::io::Write;

mod error;
mod token;
mod expr;

fn run(code: &str) {
    let scanner = token::Scanner::new(code);
    let tokens = scanner.tokenize();
    if error::had_error() {
        return;
    }
    // for token in tokens {
    //     println!("{:?}", token);
    // }
    let parser = expr::Parser::new(&tokens);
    let Ok(expr) = parser.parse() else {
        return;
    };

    println!("AST = {}", expr.print());

    let Ok(value) = expr.eval() else {
        return;
    };

    println!("= {}", value);
}


fn run_file(path: &String) {
    let code = std::fs::read_to_string(path).expect("Failed to read file");
    run(code.as_str());

    if error::had_error() {
        std::process::exit(1);
    }
}

fn run_prompt() {
    let mut code = String::new();
    loop {
        print!("{}", if code.is_empty() { "> " } else { "| " });
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).expect("Failed to read line") == 0 { break; }
        let line = line.trim_end_matches(['\r', '\n']);

        if line.ends_with('\\') {
            code.push_str(line.trim_end_matches('\\'));
            code.push('\n');
            continue;
        } else {
            code.push_str(line);
            run(code.as_str());
            code.clear();
            error::reset_had_error();
        }
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
