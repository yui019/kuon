use crate::{
    compiler::{chunk::Chunk, operation::Operation, value::Value},
    parser::expression::Expression,
};

use super::function_definition::compile_function_definition;

pub fn compile_value(
    chunk: &mut Chunk,
    is_function: bool,
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

        Expression::Identifier(v) => {
            if is_function {
                let function_index = chunk.function_index_from_name.get(v);
                if function_index.is_some() {
                    let function = Value::Function(*function_index.unwrap());
                    chunk.add_operation(&Operation::Push(function));
                } else {
                    chunk.add_operation(&Operation::Load(v.clone()));
                }
            } else {
                chunk.add_operation(&Operation::Load(v.clone()));
            }
        }

        Expression::FunctionDefinition {
            name, params, body, ..
        } => {
            let index = compile_function_definition(
                chunk,
                is_function,
                params,
                body,
                name,
            )?;

            chunk.add_operation(&Operation::Push(Value::Function(index)))
        }

        _ => unreachable!(),
    }

    Ok(())
}
