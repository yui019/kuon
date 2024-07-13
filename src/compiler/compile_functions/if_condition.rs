use crate::{
    compiler::{chunk::Chunk, compile_expression, operation::Operation},
    parser::expression::Expression,
};

pub fn compile_if_condition(
    chunk: &mut Chunk,
    is_function: bool,
    condition: &Expression,
    true_branch: &Expression,
    else_branch: &Option<Box<Expression>>,
) -> Result<(), String> {
    compile_expression(chunk, condition, is_function)?;

    chunk.add_operation(&Operation::JumpIfFalse(0));
    let jump_to_else_address = chunk.get_latest_address();

    compile_expression(chunk, true_branch, is_function)?;
    chunk.add_operation(&Operation::Jump(0));

    let jump_to_end_address = chunk.get_latest_address();

    let else_start_address = chunk.get_latest_address() + 1;

    if else_branch.is_some() {
        compile_expression(chunk, &else_branch.as_ref().unwrap(), is_function)?;
    }

    let end_address = chunk.get_latest_address() + 1;

    chunk.update_operation(
        jump_to_else_address,
        &Operation::JumpIfFalse(else_start_address),
    );
    chunk.update_operation(jump_to_end_address, &Operation::Jump(end_address));

    Ok(())
}
