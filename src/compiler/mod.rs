use chunk::Chunk;
use compile_functions::{
    if_condition::compile_if_condition, infix::compile_infix,
    prefix::compile_prefix, value::compile_value,
    variable_definition::compile_variable_definition,
};

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
            compile_expression(&mut chunk, expression)?;
        }
    } else {
        return Err(format!(
            "Source code is expected to be a block expression"
        ));
    }

    Ok(chunk)
}

fn compile_expression(
    chunk: &mut Chunk,
    expression: &Expression,
) -> Result<(), String> {
    match expression {
        value @ (Expression::Null
        | Expression::String(_)
        | Expression::Char(_)
        | Expression::Int(_)
        | Expression::Float(_)
        | Expression::Identifier(_)) => compile_value(chunk, value)?,

        Expression::Prefix { operator, value } => {
            compile_prefix(chunk, operator, &value)?
        }

        Expression::Infix {
            left,
            operator,
            right,
        } => compile_infix(chunk, left, operator, right)?,

        Expression::Postfix { .. } => todo!(),

        Expression::Block { expressions } => {
            for expression in expressions {
                compile_expression(chunk, expression)?;
            }
        }

        Expression::IfCondition {
            condition,
            true_branch,
            else_branch,
        } => compile_if_condition(chunk, condition, true_branch, else_branch)?,

        Expression::VariableDefinition { name, value, .. } => {
            compile_variable_definition(chunk, name, value)?
        }

        Expression::FunctionDefinition { .. } => todo!(),

        Expression::FunctionCall { .. } => todo!(),

        Expression::Type { .. } => unreachable!(),
    }

    Ok(())
}
