use crate::{
    analyzer::{analyzer_error::AnalyzerError, env::Environment},
    analyzer_error,
    lexer::token::TokenData,
    parser::{expression::Expression, r#type::Type},
};

use super::validate_and_get_type;

pub fn validate_prefix(
    env: &mut Environment,
    operator: &TokenData,
    value: &mut Expression,
) -> Result<Type, AnalyzerError> {
    if *operator != TokenData::Minus {
        unreachable!();
    }

    match validate_and_get_type(value, env)? {
        type_ @ (Type::Int | Type::Float) => return Ok(type_),

        type_ => {
            return analyzer_error!(
                value.line,
                "Prefix operator - can not work on an expression of type {:?}",
                type_
            )
        }
    }
}
