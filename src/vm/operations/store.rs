use std::collections::HashMap;

use crate::{
    compiler::value::{Object, Value},
    parser::expression::VariableAccessor,
    vm::{heap::VmHeap, StackValueWrapper},
};

fn execute_store_without_accessors(
    heap: &mut VmHeap,
    variables: &mut HashMap<String, Value>,
    name: &String,
    value: Value,
) {
    // if you're storing into a variable which already holds an
    // object ref, then update the object it's referencing with that
    // value
    if let Some(Value::ObjectRef(dest_index)) = variables.get(name) {
        let object = match &value {
            // if you're storing an object ref, get the object from
            // the heap
            Value::ObjectRef(src_index) => heap.get_object(*src_index),

            // otherwise, create an Object::Value
            v => Object::Value(v.clone()),
        };

        heap.update_object(*dest_index, object);
    }

    variables.insert(name.clone(), value);
}

/// Create a new value which is the same as old_value except one specific field
/// was updated to new_field_value (by tracing the accessors one by one)
fn update_value_field(
    heap: &mut VmHeap,
    old_value: &Value,
    accessors: &Vec<VariableAccessor>,
    new_field_value: &Value,
) -> Value {
    if accessors.is_empty() {
        return new_field_value.clone();
    }

    let mut new_value = old_value.clone();

    match &accessors[0] {
        VariableAccessor::StructField(field) => match &mut new_value {
            Value::Struct(fields) => {
                fields.insert(
                    field.to_string(),
                    update_value_field(
                        heap,
                        &fields[field],
                        &accessors[1..].to_vec(),
                        new_field_value,
                    ),
                );
            }

            // same as above on Value::Struct, except that instead of directly
            // updating the value, I get the object behind the reference, update
            // it and write it back to the heap
            Value::ObjectRef(index) => {
                let mut object = heap.get_object(*index);

                match &mut object {
                    Object::Value(Value::Struct(fields)) => {
                        fields.insert(
                            field.to_string(),
                            update_value_field(
                                heap,
                                &fields[field],
                                &accessors[1..].to_vec(),
                                new_field_value,
                            ),
                        );
                    }

                    v => panic!("Value is not a struct: {:?}", v),
                }

                heap.update_object(*index, object);
            }

            v => panic!("Value is not a struct: {:?}", v),
        },
    }

    new_value
}

fn execute_store_with_accessors(
    heap: &mut VmHeap,
    variables: &mut HashMap<String, Value>,
    name: &String,
    accessors: &Vec<VariableAccessor>,
    value: Value,
) {
    // if accessors is not empty, a variable needs to already exist, so
    // unwrapping here
    let current_variable_value = variables.get(name).unwrap();

    variables.insert(
        name.clone(),
        update_value_field(heap, current_variable_value, accessors, &value),
    );
}

pub fn execute_store_with_value(
    heap: &mut VmHeap,
    variables: &mut HashMap<String, Value>,
    name: &String,
    accessors: &Vec<VariableAccessor>,
    value: Value,
) {
    if accessors.is_empty() {
        execute_store_without_accessors(heap, variables, name, value);
    } else {
        execute_store_with_accessors(heap, variables, name, accessors, value);
    }
}

pub fn execute_store(
    heap: &mut VmHeap,
    stack: &mut Vec<StackValueWrapper>,
    variables: &mut HashMap<String, Value>,
    name: &String,
    accessors: &Vec<VariableAccessor>,
) {
    let value = stack.pop().unwrap();

    execute_store_with_value(heap, variables, name, accessors, value.value);
}
