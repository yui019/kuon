use crate::{
    expression, expression_pat,
    parser::{
        expression::{ExpressionData, VariableAccessor},
        parser_error::ParserError,
    },
};

use super::super::expression::Expression;

fn parse_accessors(
    left: &Expression,
) -> Result<(String, Vec<VariableAccessor>), ParserError> {
    match left {
        expression_pat!(ExpressionData::Identifier(identifier)) => {
            return Ok((identifier.clone(), vec![]))
        }

        expression_pat!(ExpressionData::FieldAccess { expression, field }) => {
            let (name, accessors) = parse_accessors(&expression)?;

            let mut new_accessors: Vec<VariableAccessor> = vec![];
            new_accessors.reserve(accessors.len() + 1);

            new_accessors.append(&mut accessors.clone());
            new_accessors.push(VariableAccessor::StructField(field.clone()));

            return Ok((name, new_accessors));
        }

        _ => {
            todo!("idk this error doesn't fit anymore")
            // return parser_error!(
            //     left.line,
            //     "Variable name should be an identifier"
            // );
        }
    };
}

/// Called on infix TokenData::Equals operator
pub fn create_variable_assignment(
    left: &Expression,
    right: &Expression,
) -> Result<Expression, ParserError> {
    let (name, accessors) = parse_accessors(left)?;

    Ok(expression!(
        VariableAssignment {
            name,
            accessors,
            value: Box::new(right.clone()),
        },
        left.line
    ))
}
