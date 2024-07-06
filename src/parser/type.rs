#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Null,
    Int,
    Uint,
    Float,
    Bool,
    Char,
    String,

    Function {
        param_types: Vec<Type>,
        return_type: Box<Type>,
    },
}
