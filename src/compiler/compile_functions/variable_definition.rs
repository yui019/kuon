use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    parser::expression::Expression,
};

pub fn compile_variable_definition(
    chunk: &mut Chunk,
    is_function: bool,
    name: &String,
    value: &Expression,
) -> Result<(), String> {
    compile_expression(chunk, value, is_function)?;
    chunk.add_operation(&Operation::Store(name.clone()));

    Ok(())
}
