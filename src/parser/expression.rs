use std::collections::HashMap;

use crate::lexer::token::TokenData;

use super::r#type::Type;

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub enum ExpressionData {
    Null,

    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Bool(bool),
    Identifier(String),

    Prefix {
        operator: TokenData,
        value: Box<Expression>,
    },

    Infix {
        left: Box<Expression>,
        operator: TokenData,
        right: Box<Expression>,
    },

    Postfix {
        value: Box<Expression>,
        operator: TokenData,
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

    VariableAssignment {
        name: String,
        value: Box<Expression>,
    },

    FunctionDefinition {
        name: Option<String>,
        params: Vec<FunctionParam>,
        return_type: Type,
        body: Box<Expression>,
    },

    StructDefinition {
        name: Option<String>,
        fields: HashMap<String, Type>,
    },

    MakeStruct {
        name: Option<String>,
        fields: HashMap<String, Expression>,
    },

    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    FieldAccess {
        expression: Box<Expression>,
        field: String,
    },

    Type {
        type_: Type,
    },
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub data: ExpressionData,
    pub line: usize,
}
