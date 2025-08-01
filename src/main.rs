use minilisp_rust::{Env, eval, parse};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut env: Env = HashMap::new();
    println!("Welcome to minilisp-rust!");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            // Ctrl-D was pressed, so exit.
            println!("Goodbye!");
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match parse(input) {
            Ok(expr) => match eval(&expr, &mut env) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
