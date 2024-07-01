use crate::{
    lexer::{token::Token, Lexer},
    parser::r#type::Type,
};

use super::super::expression::Expression;

/// Called after Token::LeftParenCurly
pub fn parse_type(lexer: &mut Lexer) -> Result<Type, String> {
    let name = lexer.next();

    match name {
        Some(
            Token::Any
            | Token::Null
            | Token::Int
            | Token::Uint
            | Token::Float
            | Token::Bool
            | Token::Char
            | Token::String,
        ) => {}

        None => return Err(format!("Expected type")),
        Some(t) => return Err(format!("Unrecognized type: {:?}", t)),
    }

    let name = name.unwrap();
    let type_ = Type { name };

    Ok(type_)
}
