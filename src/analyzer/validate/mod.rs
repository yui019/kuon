use block::validate_block;
use field_access::validate_field_access;
use function_call::validate_function_call;
use function_definition::validate_function_definition;
use identifier::validate_identifier;
use if_condition::validate_if_condition;
use infix::validate_infix;
use make_struct::validate_make_struct;
use prefix::validate_prefix;
use struct_definition::validate_struct_definition;
use variable_assignment::validate_variable_assignment;
use variable_definition::validate_variable_definition;

use crate::{
    analyzer_error, expression_pat,
    parser::{
        expression::{Expression, ExpressionData},
        r#type::Type,
    },
};

use super::{analyzer_error::AnalyzerError, env::Environment};

mod block;
mod field_access;
mod function_call;
mod function_definition;
mod identifier;
mod if_condition;
mod infix;
mod make_struct;
mod prefix;
mod struct_definition;
mod variable_assignment;
mod variable_definition;

pub fn validate_and_get_type(
    expression: &Expression,
    env: &mut Environment,
) -> Result<Type, AnalyzerError> {
    match expression {
        expression_pat!(ExpressionData::Null) => return Ok(Type::Null),

        expression_pat!(ExpressionData::String(_)) => return Ok(Type::String),

        expression_pat!(ExpressionData::Char(_)) => return Ok(Type::Char),

        expression_pat!(ExpressionData::Int(_)) => return Ok(Type::Int),

        expression_pat!(ExpressionData::Float(_)) => return Ok(Type::Float),

        expression_pat!(ExpressionData::Bool(_)) => return Ok(Type::Bool),

        expression_pat!(ExpressionData::Identifier(identifier), line) => {
            validate_identifier(env, *line, identifier)
        }

        expression_pat!(ExpressionData::Prefix { operator, value }) => {
            validate_prefix(env, operator, value)
        }

        expression_pat!(ExpressionData::Infix {
            left,
            operator,
            right,
        }) => validate_infix(env, left, operator, right),

        expression_pat!(ExpressionData::Postfix { .. }) => todo!(),

        expression_pat!(ExpressionData::Block { expressions }) => {
            validate_block(env, expressions)
        }

        expression_pat!(
            ExpressionData::IfCondition {
                condition,
                true_branch,
                else_branch,
            },
            line
        ) => validate_if_condition(
            env,
            *line,
            condition,
            true_branch,
            else_branch,
        ),

        expression_pat!(
            ExpressionData::VariableDefinition {
                type_,
                value,
                name,
                constant,
            },
            line
        ) => validate_variable_definition(
            env, *line, type_, value, name, *constant,
        ),

        expression_pat!(
            ExpressionData::VariableAssignment {
                name,
                accessors,
                value
            },
            line
        ) => validate_variable_assignment(env, *line, name, accessors, value),

        expression_pat!(
            ExpressionData::FunctionDefinition {
                name,
                params,
                return_type,
                body,
            },
            line
        ) => validate_function_definition(
            env,
            *line,
            name,
            params,
            return_type,
            body,
        ),

        expression_pat!(
            ExpressionData::StructDefinition { name, fields },
            line
        ) => validate_struct_definition(env, *line, name, fields),

        expression_pat!(ExpressionData::MakeStruct { name, fields }, line) => {
            validate_make_struct(env, *line, name, fields)
        }

        expression_pat!(ExpressionData::FunctionCall {
            function,
            arguments,
        }) => validate_function_call(env, function, arguments),

        expression_pat!(ExpressionData::FieldAccess { expression, field }) => {
            validate_field_access(env, expression, field)
        }

        expression_pat!(ExpressionData::Type { .. }, line) => {
            return analyzer_error!(*line, "Cannot use a type as an expression")
        }
    }
}
