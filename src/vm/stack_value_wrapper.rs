use crate::{compiler::value::Value, parser::expression::VariableAccessor};

#[derive(Debug, Clone, PartialEq)]
pub struct StackValueWrapperCameFrom {
    pub name: String,
    pub accessors: Vec<VariableAccessor>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackValueWrapper {
    pub value: Value,
    pub came_from: Option<StackValueWrapperCameFrom>,
}

impl StackValueWrapper {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            came_from: None,
        }
    }

    pub fn new_from_name(value: Value, name: String) -> Self {
        Self {
            value,
            came_from: Some(StackValueWrapperCameFrom {
                name,
                accessors: vec![],
            }),
        }
    }

    pub fn new_from_name_and_accessors(
        value: Value,
        name: String,
        accessors: Vec<VariableAccessor>,
    ) -> Self {
        Self {
            value,
            came_from: Some(StackValueWrapperCameFrom { name, accessors }),
        }
    }
}
