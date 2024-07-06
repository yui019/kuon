use crate::lexer::token::{Token, TokenData};

pub fn token_matches(token: &Option<Token>, token_data: &TokenData) -> bool {
    match token {
        Some(Token { data, .. }) if data == token_data => true,

        _ => false,
    }
}

#[macro_export]
macro_rules! token_data {
    ($data:pat) => {
        Token { data: $data, .. }
    };
}
