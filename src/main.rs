use std::fs;

use kuon::{lexer::Lexer, parser};

fn main() {
    let source = fs::read_to_string("test.kn").unwrap();
    let mut lexer = Lexer::from_string(&source);

    println!("\n========================================\n");

    for token in lexer.clone() {
        println!("{:?}", token);
    }

    println!("\n========================================\n");

    println!("{:#?}", parser::parse(&mut lexer));
}
