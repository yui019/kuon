use crate::{
    lexer::{
        token::{Token, TokenData::*},
        Lexer,
    },
    parser::{parser_error::ParserError, r#type::Type},
    parser_error, parser_error_eof, token_pat,
};

/// Called after Token::LeftParenCurly
pub fn parse_type(lexer: &mut Lexer) -> Result<Type, ParserError> {
    let name = lexer.next();

    let name = match name {
        Some(
            token @ token_pat!(Any | Null | Int | Float | Bool | Char | String),
        ) => token.data,

        None => return parser_error_eof!("Expected type"),
        Some(t) => {
            return parser_error!(t.line, "Unrecognized type: {:?}", t.data)
        }
    };

    let type_ = match name {
        Any => Type::Any,
        Null => Type::Null,
        Int => Type::Int,
        Float => Type::Float,
        Bool => Type::Bool,
        Char => Type::Char,
        String => Type::String,

        _ => unreachable!(),
    };

    Ok(type_)
}
