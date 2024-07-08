use std::fs;

use color_print::cprintln;
use kuon::{analyzer, compiler, lexer::Lexer, parser};

fn main() {
    let source = fs::read_to_string("test.kn").unwrap();
    let mut lexer = Lexer::from_string(&source);

    println!("\n========================================\n");

    for token in lexer.clone() {
        println!("{:?}", token);
    }

    println!("\n========================================\n");

    let mut parse_result = parser::parse_source(&mut lexer);
    match &mut parse_result {
        Ok(ref mut ast) => {
            if let Err(e) = analyzer::validate(ast) {
                cprintln!("<red>[Analyzer error]</red> {}", e);
            } else {
                println!("{:#?}", ast);
            }
        }

        Err(e) => {
            cprintln!("<red>[Parser error]</red> {:?}", e);
        }
    }

    println!("\n========================================\n");

    let ast = parse_result.unwrap();
    let bytecode_chunk = compiler::compile_source(&ast);
    println!("{:#?}", bytecode_chunk);
}
