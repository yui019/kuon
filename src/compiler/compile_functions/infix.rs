use crate::{
    compiler::{chunk::Chunk, operation::Operation},
    lexer::token::TokenData,
    parser::expression::Expression,
};

use super::value::compile_value;

pub fn compile_infix(
    chunk: &mut Chunk,
    left: &Expression,
    operator: &TokenData,
    right: &Expression,
) -> Result<(), String> {
    match operator {
        TokenData::Plus => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::Add);
        }

        TokenData::Minus => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::Substract);
        }

        TokenData::Star => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::Multiply);
        }

        TokenData::Slash => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::Divide);
        }

        TokenData::EqualsEquals => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::Equal);
        }

        TokenData::LessThan => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::LessThan);
        }

        TokenData::LessThanOrEqual => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::LessThanOrEqual);
        }

        TokenData::GreaterThan => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::GreaterThan);
        }

        TokenData::GreaterThanOrEqual => {
            compile_value(chunk, left)?;
            compile_value(chunk, right)?;
            chunk.add_operation(&Operation::GreaterThanOrEqual);
        }

        _ => unreachable!(),
    }

    Ok(())
}
