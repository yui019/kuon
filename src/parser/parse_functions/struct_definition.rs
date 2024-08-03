use std::collections::BTreeMap;

use crate::{
    expression,
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::{parser_error::ParserError, r#type::Type, util::token_matches},
    parser_error, parser_error_eof, some_token_pat,
};

use super::{super::expression::Expression, r#type::parse_type};

pub fn parse_fields(
    lexer: &mut Lexer,
) -> Result<BTreeMap<String, Type>, ParserError> {
    let mut fields: BTreeMap<String, Type> = BTreeMap::new();

    loop {
        let next = lexer.next();

        if token_matches(&next, &TokenData::RightParenCurly) {
            break;
        }

        match next {
            some_token_pat!(TokenData::ValueIdentifier(identifier), line) => {
                let name = identifier;
                let type_ = parse_type(lexer)?;

                // don't allow duplicate fields
                if fields.contains_key(&name) {
                    return parser_error!(
                        line,
                        "Duplicate struct field: {}",
                        name
                    );
                }

                fields.insert(name, type_);

                // skip optional comma
                if matches!(lexer.peek(), some_token_pat!(TokenData::Comma)) {
                    lexer.next();
                }
            }

            Some(t) => {
                return parser_error!(
                    t.line,
                    "Expected identifier, got {:?}",
                    t.data
                )
            }
            None => return parser_error_eof!("Expected field"),
        }
    }

    return Ok(fields);
}

/// Called after Token::Struct
pub fn parse_struct_definition(
    lexer: &mut Lexer,
    top_level: bool,
    line: usize,
) -> Result<Expression, ParserError> {
    let mut name: Option<String> = None;

    // get name if it exists
    let first_token = lexer.next();
    match first_token.clone() {
        some_token_pat!(TokenData::LeftParenCurly) => {}

        some_token_pat!(TokenData::ValueIdentifier(identifier)) => {
            name = Some(identifier);

            let next = lexer.next();
            if !token_matches(&next, &TokenData::LeftParenCurly) {
                return parser_error!(
                    next.unwrap().line,
                    "Expected {{ after struct name",
                );
            }
        }

        Some(t) => {
            return parser_error!(
                t.line,
                "Expected struct name or {{, got {:?}",
                t.data
            )
        }
        None => return parser_error_eof!("Expected struct name or {{"),
    }

    if !top_level && name.is_some() {
        return parser_error!(
            first_token.unwrap().line,
            "Only top level struct definitions can have a name",
        );
    }

    let fields = parse_fields(lexer)?;

    Ok(expression!(StructDefinition { name, fields }, line))
}
