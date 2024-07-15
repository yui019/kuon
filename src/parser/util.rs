use crate::lexer::token::{Token, TokenData};

pub fn token_matches(token: &Option<Token>, token_data: &TokenData) -> bool {
    match token {
        Some(Token { data, .. }) if data == token_data => true,

        _ => false,
    }
}

#[macro_export]
macro_rules! token_pat {
    ($data:pat, $line: pat) => {
        Token {
            data: $data,
            line: $line,
        }
    };

    ($data:pat) => {
        Token { data: $data, .. }
    };
}

#[macro_export]
macro_rules! some_token_pat {
    ($data:pat, $line: pat) => {
        Some(Token {
            data: $data,
            line: $line,
        })
    };

    ($data:pat) => {
        Some(Token { data: $data, .. })
    };
}

#[macro_export]
macro_rules! expression {
    ($data:expr, $line: expr) => {{
        use crate::parser::expression::ExpressionData::*;

        Expression {
            data: $data,
            line: $line,
        }
    }};
}

#[macro_export]
macro_rules! expression_pat {
    ($data:pat, $line: pat) => {
        Expression {
            data: $data,
            line: $line,
        }
    };

    ($data:pat) => {
        Expression { data: $data, .. }
    };
}
