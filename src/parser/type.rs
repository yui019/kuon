use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: Token,
}
