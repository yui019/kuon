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

use crate::{
    expression_pat,
    parser::expression::{Expression, ExpressionData},
};

pub mod chunk;
mod compile_functions;
pub mod operation;
pub mod value;

pub fn compile_source(ast: &Expression) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();

    if let expression_pat!(ExpressionData::Block { expressions }) = ast {
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
    use ExpressionData::*;

    match expression {
        value @ Expression {data: ExpressionData::Null
        | ExpressionData::String(_)
        | ExpressionData::Char(_)
        | ExpressionData::Int(_)
        | ExpressionData::Float(_)
        | ExpressionData::Bool(_)
        | ExpressionData::Identifier(_)
        // function definitions without names are closures, so they are handled like all values
        | ExpressionData::FunctionDefinition { name: None, .. }
        | ExpressionData::MakeStruct { .. }
        | ExpressionData::FieldAccess { .. }, ..} => {
            compile_value(chunk, is_function, value)?
        }

        expression_pat!(Prefix { operator, value }) => {
            compile_prefix(chunk, is_function, operator, &value)?
        }

        expression_pat!(Infix {
            left,
            operator,
            right,
        }) => compile_infix(chunk, is_function, left, operator, right)?,

        // no Postfix expressions yet
        expression_pat!(Postfix { .. }) => {}

        expression_pat!(Block { expressions }) => {
            for expression in expressions {
                compile_expression(chunk, expression, is_function)?;
            }
        }

        expression_pat!(IfCondition {
            condition,
            true_branch,
            else_branch,
        }) => compile_if_condition(chunk, is_function, condition, true_branch, else_branch)?,

        expression_pat!(VariableDefinition { name, value, .. }) => {
            compile_variable_definition(chunk, is_function, name, value)?
        }

        expression_pat!(VariableAssignment { name, accessors, value }) => {
            compile_variable_assignment(chunk, is_function, name, accessors, value)?
        }

        expression_pat!(FunctionDefinition {
            params, body, name, ..
        }) => {
            compile_function_definition(chunk, is_function, params, body, name)?;
            ()
        }

        expression_pat!(ExpressionData::StructDefinition { .. }) => {}
        
        expression_pat!(FunctionCall { function, arguments }) => compile_function_call(chunk, is_function, function, arguments)?,

        // this should be unreachable unless I seriously mess something up
        expression_pat!(Type { .. }) => unreachable!(),
    }

    Ok(())
}
