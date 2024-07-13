use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    lexer::token::TokenData,
    parser::expression::Expression,
};

pub fn compile_prefix(
    chunk: &mut Chunk,
    is_function: bool,
    operator: &TokenData,
    value: &Expression,
) -> Result<(), String> {
    match operator {
        TokenData::Minus => {
            compile_expression(chunk, value, is_function)?;
            chunk.add_operation(&Operation::Negate);
        }

        _ => unreachable!(),
    }

    Ok(())
}
