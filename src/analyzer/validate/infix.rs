use crate::{
    analyzer::{
        analyzer_error::AnalyzerError, env::Environment, util::types_equal,
    },
    analyzer_error,
    lexer::token::TokenData,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_infix(
    env: &mut Environment,
    left: &mut Expression,
    operator: &TokenData,
    right: &mut Expression,
) -> Result<Type, AnalyzerError> {
    if *operator == TokenData::Minus
        || *operator == TokenData::Plus
        || *operator == TokenData::Star
        || *operator == TokenData::Slash
    {
        let left_type = validate_and_get_type(left, env)?;
        let right_type = validate_and_get_type(right, env)?;

        match (left_type, right_type) {
            (Type::Int, Type::Int) => {
                return if *operator == TokenData::Slash {
                    Ok(Type::Float)
                } else {
                    Ok(Type::Int)
                };
            }

            (Type::Int, Type::Float) => {
                return Ok(Type::Float);
            }

            (Type::Float, Type::Float) => {
                return Ok(Type::Float);
            }

            (Type::Float, Type::Int) => {
                return Ok(Type::Float);
            }

            // This is here temporarily to let me test out generics with a
            // simple add function. Instead, this is supposed to check if the
            // type implements some arithmetics interface or something (but
            // interfaces don't exist yet!!)
            (left_type, right_type)
                if types_equal(env, &left_type, &right_type) =>
            {
                return Ok(left_type)
            }
            _ => {
                return analyzer_error!(
                    left.line,
                    "Operator {:?} cannot work on operators of different types",
                    operator
                )
            } /* _ => {
               *     return analyzer_error!(
               *         left.line,
               *         "Operator {:?} only works on numbers",
               *         operator
               *     )
               * } */
        }
    } else if *operator == TokenData::LessThan
        || *operator == TokenData::LessThanOrEqual
        || *operator == TokenData::GreaterThan
        || *operator == TokenData::GreaterThanOrEqual
    {
        let left_type = validate_and_get_type(left, env)?;
        let right_type = validate_and_get_type(right, env)?;

        match (left_type, right_type) {
            (Type::Int | Type::Float, Type::Int | Type::Float) => {
                return Ok(Type::Bool);
            }

            _ => {
                return analyzer_error!(
                    left.line,
                    "Operator {:?} only works on numbers",
                    operator
                )
            }
        }
    } else if *operator == TokenData::EqualsEquals {
        let left_type = validate_and_get_type(left, env)?;
        let right_type = validate_and_get_type(right, env)?;

        if types_equal(env, &left_type, &right_type) {
            return Ok(Type::Bool);
        } else {
            return analyzer_error!(
                left.line,
                "Operator {:?} only works on operands of the same type",
                operator
            );
        }
    } else {
        unreachable!();
    }
}
