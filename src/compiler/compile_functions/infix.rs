use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    lexer::token::TokenData,
    parser::expression::Expression,
};

pub fn compile_infix(
    chunk: &mut Chunk,
    left: &Expression,
    operator: &TokenData,
    right: &Expression,
) -> Result<(), String> {
    match operator {
        TokenData::Plus => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::Add);
        }

        TokenData::Minus => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::Substract);
        }

        TokenData::Star => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::Multiply);
        }

        TokenData::Slash => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::Divide);
        }

        TokenData::EqualsEquals => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::Equal);
        }

        TokenData::LessThan => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::LessThan);
        }

        TokenData::LessThanOrEqual => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::LessThanOrEqual);
        }

        TokenData::GreaterThan => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::GreaterThan);
        }

        TokenData::GreaterThanOrEqual => {
            compile_expression(chunk, left)?;
            compile_expression(chunk, right)?;
            chunk.add_operation(&Operation::GreaterThanOrEqual);
        }

        _ => unreachable!(),
    }

    Ok(())
}
