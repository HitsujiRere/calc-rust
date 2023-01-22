mod ast;
mod evaler;
mod parser;

use std::io::stdin;

fn main() {
    let mut evaler = evaler::Evaler::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        match parser::parse(&input) {
            Ok((_, expr)) => match evaler.eval(&expr) {
                Ok(res) => println!("> {}", res),
                Err(err) => println!("> {}", err),
            },
            _ => println!("Syntax Error!"),
        }
    }
}
