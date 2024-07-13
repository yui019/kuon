use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    parser::expression::Expression,
};

/// Returns function index if successful
pub fn compile_function_call(
    chunk: &mut Chunk,
    is_function: bool,
    function: &Expression,
    arguments: &Vec<Expression>,
) -> Result<(), String> {
    for argument in arguments {
        compile_expression(chunk, argument, is_function)?;
    }

    compile_expression(chunk, function, true)?;

    chunk.add_operation(&Operation::Call);

    Ok(())
}
