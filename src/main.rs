mod ast;
mod evaler;
mod parser;

use std::io::stdin;

fn main() {
    let evaler = evaler::Evaler::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        match parser::parse(&input) {
            Ok((_, expr)) => println!("{}", evaler.eval(&expr)),
            _ => println!("Syntax Error!"),
        }
    }
}
