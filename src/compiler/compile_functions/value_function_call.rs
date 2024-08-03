use crate::{
    compiler::{
        chunk::Chunk, compile_expression, operation::Operation, value::Value,
    },
    parser::{expression::Expression, r#type::Type},
};

/// Returns function index if successful
pub fn compile_value_function_call(
    chunk: &mut Chunk,
    is_function: bool,
    pre_argument: &Expression,
    function_name: &String,
    arguments: &Vec<Expression>,
    pre_argument_type: &Type,
) -> Result<(), String> {
    compile_expression(chunk, pre_argument, is_function)?;

    for argument in arguments {
        compile_expression(chunk, argument, is_function)?;
    }

    let key = (function_name.clone(), pre_argument_type.clone());
    let function_index = chunk.value_function_index_from_name[&key];

    chunk.add_operation(&Operation::Push(Value::Function(function_index)));
    chunk.add_operation(&Operation::Call);

    Ok(())
}
