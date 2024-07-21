use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Null,
    Int,
    Float,
    Bool,
    Char,
    String,

    Function {
        param_types: Vec<Type>,
        return_type: Box<Type>,
    },

    Struct {
        fields: HashMap<String, Type>,
    },

    UserDefined(String),
}
