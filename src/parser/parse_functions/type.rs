use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{parser_error::ParserError, r#type::Type},
    parser_error, parser_error_eof, token_data,
};

/// Called after Token::LeftParenCurly
pub fn parse_type(lexer: &mut Lexer) -> Result<Type, ParserError> {
    let name = lexer.next();

    let name = match name {
        Some(
            token @ token_data!(
                TokenData::Any
                    | TokenData::Null
                    | TokenData::Int
                    | TokenData::Uint
                    | TokenData::Float
                    | TokenData::Bool
                    | TokenData::Char
                    | TokenData::String
            ),
        ) => token.data,

        None => return Err(parser_error_eof!("Expected type")),
        Some(t) => {
            return Err(parser_error!(
                t.line,
                "Unrecognized type: {:?}",
                t.data
            ))
        }
    };

    let type_ = match name {
        TokenData::Any => Type::Any,
        TokenData::Null => Type::Null,
        TokenData::Int => Type::Int,
        TokenData::Uint => Type::Uint,
        TokenData::Float => Type::Float,
        TokenData::Bool => Type::Bool,
        TokenData::Char => Type::Char,
        TokenData::String => Type::String,

        _ => unreachable!(),
    };

    Ok(type_)
}
