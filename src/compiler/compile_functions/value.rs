use crate::{
    compiler::{
        chunk::Chunk, compile_expression, operation::Operation, value::Value,
    },
    parser::expression::Expression,
};

pub fn compile_value(
    chunk: &mut Chunk,
    value: &Expression,
) -> Result<(), String> {
    match value {
        Expression::Null => chunk.add_operation(&Operation::Push(Value::Null)),
        Expression::String(v) => {
            chunk.add_operation(&Operation::Push(Value::String(v.clone())))
        }
        Expression::Char(v) => {
            chunk.add_operation(&Operation::Push(Value::Char(v.clone())))
        }
        Expression::Int(v) => {
            chunk.add_operation(&Operation::Push(Value::Int(v.clone())))
        }
        Expression::Float(v) => {
            chunk.add_operation(&Operation::Push(Value::Float(v.clone())))
        }

        Expression::Identifier(_) => todo!(),

        _ => compile_expression(chunk, value)?,
    }

    Ok(())
}
