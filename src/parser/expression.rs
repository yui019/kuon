use std::collections::{BTreeMap, HashMap};

use crate::lexer::token::TokenData;

use super::r#type::Type;

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub type_: Type,
    pub constant: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableAccessor {
    StructField(String),
    // TODO: add array indexing
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
        accessors: Vec<VariableAccessor>,
        value: Box<Expression>,
    },

    FunctionDefinition {
        name: Option<String>,
        pre_parameter: Option<FunctionParam>,
        params: Vec<FunctionParam>,
        return_type: Type,
        body: Box<Expression>,
    },

    StructDefinition {
        name: Option<String>,
        fields: BTreeMap<String, Type>,
    },

    MakeStruct {
        name: Option<String>,
        fields: HashMap<String, Expression>,
    },

    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    ValueFunctionCall {
        pre_argument: Box<Expression>,
        function_name: String,
        arguments: Vec<Expression>,

        // This field will always be None in the AST produced by the parser.
        // It's filled by the analyzer instead
        pre_argument_type: Option<Type>,
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
