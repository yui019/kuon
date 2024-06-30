use crate::lexer::token::Token;

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
        name: Token,
        value: Box<Expression>,
        type_: Option<Box<Expression>>,
    },

    Type {
        name: Token,
    },
}