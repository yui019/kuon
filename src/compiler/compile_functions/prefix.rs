use crate::{
    compiler::{chunk::Chunk, operation::Operation},
    lexer::token::TokenData,
    parser::expression::Expression,
};

use super::value::compile_value;

pub fn compile_prefix(
    chunk: &mut Chunk,
    operator: &TokenData,
    value: &Expression,
) -> Result<(), String> {
    match operator {
        TokenData::Minus => {
            compile_value(chunk, value);
            chunk.add_operation(&Operation::Negate);
        }

        _ => unreachable!(),
    }

    Ok(())
}
