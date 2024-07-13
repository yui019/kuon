use chunk::Chunk;
use compile_functions::{
    function_call::compile_function_call,
    function_definition::compile_function_definition,
    if_condition::compile_if_condition, infix::compile_infix,
    prefix::compile_prefix, value::compile_value,
    variable_assignment::compile_variable_assignment,
    variable_definition::compile_variable_definition,
};
use operation::Operation;

use crate::parser::expression::Expression;

pub mod chunk;
mod compile_functions;
pub mod operation;
pub mod value;

pub fn compile_source(ast: &Expression) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();

    if let Expression::Block { expressions } = ast {
        // TODO: handle top level function definitions specially here
        for expression in expressions {
            compile_expression(&mut chunk, expression, false)?;
        }
    } else {
        return Err(format!(
            "Source code is expected to be a block expression"
        ));
    }

    chunk.add_operation(&Operation::Halt);

    Ok(chunk)
}

fn compile_expression(
    chunk: &mut Chunk,
    expression: &Expression,
    is_function: bool, /* is the expression a function? This is set to false
                        * and passed down usually, unless compiling a value
                        * which you know is a function */
) -> Result<(), String> {
    match expression {
        value @ (Expression::Null
        | Expression::String(_)
        | Expression::Char(_)
        | Expression::Int(_)
        | Expression::Float(_)
        | Expression::Identifier(_)
        // function definitions without names are closures, so they are handled like all values
        | Expression::FunctionDefinition { name: None, .. }) => {
            compile_value(chunk, is_function, value)?
        }

        Expression::Prefix { operator, value } => {
            compile_prefix(chunk, is_function, operator, &value)?
        }

        Expression::Infix {
            left,
            operator,
            right,
        } => compile_infix(chunk, is_function, left, operator, right)?,

        // no Postfix expressions yet
        Expression::Postfix { .. } => {}

        Expression::Block { expressions } => {
            for expression in expressions {
                compile_expression(chunk, expression, is_function)?;
            }
        }

        Expression::IfCondition {
            condition,
            true_branch,
            else_branch,
        } => compile_if_condition(chunk, is_function, condition, true_branch, else_branch)?,

        Expression::VariableDefinition { name, value, .. } => {
            compile_variable_definition(chunk, is_function, name, value)?
        }

        Expression::VariableAssignment { name, value } => {
            compile_variable_assignment(chunk, is_function, name, value)?
        }

        Expression::FunctionDefinition {
            params, body, name, ..
        } => {
            compile_function_definition(chunk, is_function, params, body, name)?;
            ()
        }

        Expression::FunctionCall { function, arguments } => compile_function_call(chunk, is_function, function, arguments)?,

        // this should be unreachable unless I seriously mess something up
        Expression::Type { .. } => unreachable!(),
    }

    Ok(())
}
