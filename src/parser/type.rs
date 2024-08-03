use std::{collections::BTreeMap, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        // Using a BTreeMap because HashMap doesn't implement Hash (or at least
        // not yet, there have been a couple of attempts and lots of
        // discussions about implementing it, it's pretty interesting)
        fields: BTreeMap<String, Type>,
    },

    UserDefined(String),
}
