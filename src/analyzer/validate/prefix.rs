use crate::{
    analyzer::env::Environment,
    lexer::token::TokenData,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_prefix(
    env: &mut Environment,
    operator: &TokenData,
    value: &Expression,
) -> Result<Type, String> {
    if *operator != TokenData::Minus {
        unreachable!();
    }

    match validate_and_get_type(value, env)? {
        type_ @ (Type::Int | Type::Float) => return Ok(type_),

        type_ => {
            return Err(format!(
                "Prefix operator - can not work on an expression of type {:?}",
                type_
            ))
        }
    }
}
