use block::validate_block;
use function_call::validate_function_call;
use function_definition::validate_function_definition;
use identifier::validate_identifier;
use if_condition::validate_if_condition;
use infix::validate_infix;
use prefix::validate_prefix;
use variable_assignment::validate_variable_assignment;
use variable_definition::validate_variable_definition;

use crate::parser::{expression::Expression, r#type::Type};

use super::env::Environment;

mod block;
mod function_call;
mod function_definition;
mod identifier;
mod if_condition;
mod infix;
mod prefix;
mod variable_assignment;
mod variable_definition;

pub fn validate_and_get_type(
    expression: &Expression,
    env: &mut Environment,
) -> Result<Type, String> {
    match expression {
        Expression::Null => return Ok(Type::Null),

        Expression::String(_) => return Ok(Type::String),

        Expression::Char(_) => return Ok(Type::Char),

        Expression::Int(_) => return Ok(Type::Int),

        Expression::Float(_) => return Ok(Type::Float),

        Expression::Bool(_) => return Ok(Type::Bool),

        Expression::Identifier(identifier) => {
            validate_identifier(env, identifier)
        }

        Expression::Prefix { operator, value } => {
            validate_prefix(env, operator, value)
        }

        Expression::Infix {
            left,
            operator,
            right,
        } => validate_infix(env, left, operator, right),

        Expression::Postfix { .. } => todo!(),

        Expression::Block { expressions } => validate_block(env, expressions),

        Expression::IfCondition {
            condition,
            true_branch,
            else_branch,
        } => validate_if_condition(env, condition, true_branch, else_branch),

        Expression::VariableDefinition {
            type_,
            value,
            name,
            constant,
        } => validate_variable_definition(env, type_, value, name, *constant),

        Expression::VariableAssignment { name, value } => {
            validate_variable_assignment(env, name, value)
        }

        Expression::FunctionDefinition {
            name,
            params,
            return_type,
            body,
        } => validate_function_definition(env, name, params, return_type, body),

        Expression::FunctionCall {
            function,
            arguments,
        } => validate_function_call(env, function, arguments),

        Expression::Type { .. } => {
            return Err(format!("Cannot use a type as an expression"))
        }
    }
}
