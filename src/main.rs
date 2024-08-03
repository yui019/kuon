use std::fs;

use color_print::cprintln;
use kuon::{analyzer, compiler, lexer::Lexer, parser, vm::execute};

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
            println!("{:#?}", ast);
            if let Err(e) = analyzer::validate(ast) {
                cprintln!("<red>[Analyzer error]</red> {:?}", e);
                return;
            } else {
                println!("{:#?}", ast);
            }
        }

        Err(e) => {
            cprintln!("<red>[Parser error]</red> {:?}", e);
            return;
        }
    }

    println!("\n========================================\n");

    let ast = parse_result.unwrap();
    let compile_result = compiler::compile_source(&ast);
    match compile_result {
        Ok(chunk) => {
            for (index, function) in chunk.functions.iter().enumerate() {
                println!("FUNCTION {}:", index);

                for (index, operation) in function.chunk.code.iter().enumerate()
                {
                    println!("{:5} | {:?}", index, operation)
                }

                println!("");
            }

            println!("CODE:");
            for (index, operation) in chunk.code.iter().enumerate() {
                println!("{:5} | {:?}", index, operation)
            }

            println!("\n========================================\n");

            println!("RESULT: {:?}", execute(&chunk));
        }

        Err(e) => {
            cprintln!("<red>[Compiler error]</red> {:?}", e);
            return;
        }
    }
}
