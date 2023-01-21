use std::io::stdin;

use ast::Eval;

mod ast;
mod parser;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        match parser::parse(&input) {
            Ok((_, expr)) => println!("{}", expr.eval()),
            _ => println!("Syntax Error!"),
        }
    }
}
