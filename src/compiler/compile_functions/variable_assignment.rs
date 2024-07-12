use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    parser::expression::Expression,
};

pub fn compile_variable_assignment(
    chunk: &mut Chunk,
    name: &String,
    value: &Expression,
) -> Result<(), String> {
    compile_expression(chunk, value)?;
    chunk.add_operation(&Operation::Store(name.clone()));

    Ok(())
}
