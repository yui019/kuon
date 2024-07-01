use crate::lexer::token::Token;

use super::r#type::Type;

#[derive(Debug, Clone)]
pub enum Expression {
    Null,

    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Identifier(String),

    Prefix {
        operator: Token,
        value: Box<Expression>,
    },

    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },

    Postfix {
        value: Box<Expression>,
        operator: Token,
    },

    Block {
        expressions: Vec<Expression>,
    },

    IfCondition {
        condition: Box<Expression>,
        true_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },

    VariableDefinition {
        constant: bool,
        name: String,
        value: Box<Expression>,
        type_: Option<Box<Type>>,
    },

    Type {
        type_: Type,
    },
}
