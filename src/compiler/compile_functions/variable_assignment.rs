use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    parser::expression::{Expression, VariableAccessor},
};

pub fn compile_variable_assignment(
    chunk: &mut Chunk,
    is_function: bool,
    name: &String,
    accessors: &Vec<VariableAccessor>,
    value: &Expression,
) -> Result<(), String> {
    compile_expression(chunk, value, is_function)?;
    chunk.add_operation(&Operation::Store {
        name: name.clone(),
        accessors: accessors.clone(),
    });

    Ok(())
}
