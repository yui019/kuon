use std::fs;

use kuon::lexer::Lexer;

fn main() {
    let source = fs::read_to_string("test.kn").unwrap();
    let lexer = Lexer::from_string(&source);

    for token in lexer {
        println!("{:?}", token);
    }
}
