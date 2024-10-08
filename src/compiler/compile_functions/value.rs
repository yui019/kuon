use crate::{
    compiler::{
        chunk::Chunk,
        compile_expression,
        operation::Operation,
        value::{Object, Value},
    },
    expression_pat,
    parser::expression::{Expression, ExpressionData},
};

use super::function_definition::compile_function_definition;

pub fn compile_value(
    chunk: &mut Chunk,
    is_function: bool,
    value: &Expression,
) -> Result<(), String> {
    use ExpressionData::*;

    match value {
        expression_pat!(Null) => {
            chunk.add_operation(&Operation::Push(Value::Null))
        }
        expression_pat!(ExpressionData::String(v)) => chunk
            .add_operation(&Operation::PushObject(Object::String(v.clone()))),
        expression_pat!(Char(v)) => {
            chunk.add_operation(&Operation::Push(Value::Char(v.clone())))
        }
        expression_pat!(Int(v)) => {
            chunk.add_operation(&Operation::Push(Value::Int(v.clone())))
        }
        expression_pat!(Float(v)) => {
            chunk.add_operation(&Operation::Push(Value::Float(v.clone())))
        }
        expression_pat!(Bool(v)) => {
            chunk.add_operation(&Operation::Push(Value::Bool(v.clone())))
        }

        expression_pat!(Identifier(v)) => {
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

        expression_pat!(FunctionDefinition {
            name,
            params,
            body,
            pre_parameter,
            ..
        }) => {
            let index = compile_function_definition(
                chunk,
                is_function,
                pre_parameter,
                params,
                body,
                name,
            )?;

            chunk.add_operation(&Operation::Push(Value::Function(index)))
        }

        expression_pat!(MakeStruct { fields, .. }) => {
            for (name, value) in fields {
                chunk.add_operation(&Operation::Push(Value::StructFieldName(
                    name.clone(),
                )));

                compile_expression(chunk, value, is_function)?;
            }

            chunk.add_operation(&Operation::MakeStruct(fields.len()));
        }

        expression_pat!(FieldAccess { expression, field }) => {
            compile_expression(chunk, expression, is_function)?;

            chunk.add_operation(&Operation::AccessField(field.clone()));
        }

        _ => unreachable!(),
    }

    Ok(())
}
