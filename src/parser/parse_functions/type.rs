use crate::{
    expression_pat,
    lexer::{
        token::{Token, TokenData::*},
        Lexer,
    },
    parser::{
        expression::{Expression, ExpressionData},
        parser_error::ParserError,
        r#type::Type,
    },
    parser_error, parser_error_eof, some_token_pat,
};

use super::struct_definition::parse_struct_definition;

/// Called after Token::LeftParenCurly
pub fn parse_type(lexer: &mut Lexer) -> Result<Type, ParserError> {
    let type_ = match lexer.next() {
        some_token_pat!(Any) => Type::Any,
        some_token_pat!(Null) => Type::Null,
        some_token_pat!(Int) => Type::Int,
        some_token_pat!(Float) => Type::Float,
        some_token_pat!(Bool) => Type::Bool,
        some_token_pat!(Char) => Type::Char,
        some_token_pat!(String) => Type::String,
        some_token_pat!(ValueIdentifier(name)) => Type::UserDefined(name),

        some_token_pat!(Struct, line) => {
            let struct_definition =
                parse_struct_definition(lexer, false, line)?;

            match struct_definition {
                expression_pat!(ExpressionData::StructDefinition {
                    fields,
                    ..
                }) => Type::Struct { fields },

                _ => unreachable!(),
            }
        }

        None => return parser_error_eof!("Expected type"),
        Some(t) => {
            return parser_error!(t.line, "Unrecognized type: {:?}", t.data)
        }
    };

    Ok(type_)
}
