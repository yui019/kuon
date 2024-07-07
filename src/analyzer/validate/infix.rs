use crate::{
    analyzer::env::Environment,
    lexer::token::TokenData,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_infix(
    env: &mut Environment,
    left: &Expression,
    operator: &TokenData,
    right: &Expression,
) -> Result<Type, String> {
    if *operator == TokenData::Minus
        || *operator == TokenData::Plus
        || *operator == TokenData::Star
        || *operator == TokenData::Slash
    {
        let left_type = validate_and_get_type(left, env)?;
        let right_type = validate_and_get_type(right, env)?;

        match (left_type, right_type) {
            (Type::Uint, Type::Uint) if *operator != TokenData::Minus => {
                return Ok(Type::Uint);
            }

            (
                Type::Int | Type::Uint | Type::Float,
                Type::Int | Type::Uint | Type::Float,
            ) => {
                return Ok(Type::Int);
            }

            _ => {
                return Err(format!(
                    "Operator {:?} only works on numbers",
                    operator
                ))
            }
        }
    } else if *operator == TokenData::LessThan
        || *operator == TokenData::LessThanOrEqual
        || *operator == TokenData::GreaterThan
        || *operator == TokenData::GreaterThanOrEqual
    {
        let left_type = validate_and_get_type(left, env)?;
        let right_type = validate_and_get_type(right, env)?;

        match (left_type, right_type) {
            (
                Type::Int | Type::Uint | Type::Float,
                Type::Int | Type::Uint | Type::Float,
            ) => {
                return Ok(Type::Bool);
            }

            _ => {
                return Err(format!(
                    "Operator {:?} only works on numbers",
                    operator
                ))
            }
        }
    } else if *operator == TokenData::EqualsEquals {
        let left_type = validate_and_get_type(left, env)?;
        let right_type = validate_and_get_type(right, env)?;

        if left_type == right_type {
            return Ok(Type::Bool);
        } else {
            return Err(format!(
                "Operator {:?} only works on operands of the same type",
                operator
            ));
        }
    } else {
        unreachable!();
    }
}
