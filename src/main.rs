use std::env;
use std::io::Write;

mod error;
mod token;
mod expr;

fn run(code: &str) {
    let scanner = token::Scanner::new(code);
    let tokens = scanner.tokenize();
    // for token in tokens {
    //     println!("{:?}", token);
    // }
    let parser = expr::Parser::new(&tokens);
    let expr = parser.parse();
    println!("AST = {}", expr.print());
    match expr.eval() {
        Ok(value) => println!("= {:?}", value),
        Err(e) => eprintln!("Evaluation error: {}", e),
    }
}


fn run_file(path: &String) {
    let code = std::fs::read_to_string(path).expect("Failed to read file");
    run(code.as_str());

    if error::had_error() {
        std::process::exit(1);
    }
}

fn run_prompt() {
    loop {
        let mut code = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        if std::io::stdin().read_line(&mut code).expect("Failed to read line") == 0 { break; }
        run(code.as_str());
        error::reset_had_error();
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
