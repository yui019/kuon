use crate::{
    lexer::{
        token::{Token, TokenData},
        Lexer,
    },
    parser::parser_error::ParserError,
    parser_error, parser_error_eof, some_token_pat,
};

/// Called after Token::LeftParenSquare
pub fn parse_generic_parameters(
    lexer: &mut Lexer,
    line: usize,
) -> Result<Vec<String>, ParserError> {
    let mut params: Vec<String> = vec![];

    loop {
        match lexer.next() {
            some_token_pat!(TokenData::RightParenSquare) => {
                break;
            }

            some_token_pat!(TokenData::ValueIdentifier(name)) => {
                params.push(name);
            }

            Some(t) => {
                return parser_error!(
                    t.line,
                    "Expected identifier or ], got {:?}",
                    t.data
                )
            }
            None => return parser_error_eof!("Expected identifier or ]"),
        }
    }

    Ok(params)
}
