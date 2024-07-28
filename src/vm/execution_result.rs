use std::collections::HashMap;

use crate::compiler::{
    chunk::Chunk,
    value::{Object, Value},
};

use super::heap::VmHeap;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionResult {
    Null,
    Char(char),
    Int(i64),
    Float(f64),
    Bool(bool),

    String(String),

    Function {
        name: Option<String>,
        index: usize,
    },

    Struct {
        fields: HashMap<String, ExecutionResult>,
    },
}

impl ExecutionResult {
    pub fn from_value(chunk: &Chunk, heap: &VmHeap, value: Value) -> Self {
        match value {
            Value::Null => ExecutionResult::Null,
            Value::Char(v) => ExecutionResult::Char(v),
            Value::Int(v) => ExecutionResult::Int(v),
            Value::Float(v) => ExecutionResult::Float(v),
            Value::Bool(v) => ExecutionResult::Bool(v),

            Value::ObjectRef(index) => {
                let object = heap.get_object(index);
                ExecutionResult::from_object(chunk, heap, object)
            }

            Value::Function(index) => {
                let mut name: Option<String> = None;

                for (key, value) in chunk.function_index_from_name.iter() {
                    if *value == index {
                        name = Some(key.clone());
                    }
                }

                ExecutionResult::Function { name, index }
            }

            Value::Struct(struct_fields) => {
                let mut result_fields: HashMap<String, ExecutionResult> =
                    HashMap::new();

                for (name, value) in struct_fields {
                    result_fields.insert(
                        name,
                        ExecutionResult::from_value(chunk, heap, value),
                    );
                }

                ExecutionResult::Struct {
                    fields: result_fields,
                }
            }

            // A StructFieldName is a component of a struct, so it shouldn't
            // ever be an execution result
            Value::StructFieldName(_) => panic!("Wtf this shouldn't happen"),
        }
    }

    pub fn from_object(chunk: &Chunk, heap: &VmHeap, object: Object) -> Self {
        match object {
            Object::String(v) => ExecutionResult::String(v),

            Object::Value(v) => ExecutionResult::from_value(chunk, heap, v),
        }
    }
}
