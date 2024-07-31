use std::collections::HashMap;

use crate::{
    compiler::{
        chunk::Chunk,
        value::{Object, Value},
    },
    vm::{
        execute_chunk, heap::VmHeap,
        operations::store::execute_store_with_value,
        stack_value_wrapper::StackValueWrapperCameFrom, StackValueWrapper,
    },
};

pub fn execute_call(
    chunk: &Chunk,
    heap: &mut VmHeap,
    stack: &mut Vec<StackValueWrapper>,
    variables: &mut HashMap<String, Value>,
) {
    let function_index = match stack.pop() {
        Some(StackValueWrapper {
            value: Value::Function(index),
            ..
        }) => index,

        _ => unreachable!(),
    };
    let function = &chunk.functions[function_index];

    // default stack which will be passed to the function
    let mut default_stack: Vec<StackValueWrapper> = vec![];

    // Array of bindings between where variables came from and object ref
    // indexes. When the function has finished executing, the variables will be
    // updated to the values inside those objects
    let mut variables_to_be_updated: Vec<(StackValueWrapperCameFrom, usize)> =
        vec![];

    // pop variables from stack into default_stack
    for param in &function.parameters {
        let value = stack.pop().unwrap();

        // if the parameter is not a constant (has a var in front in
        // the function definition) and it's not already an object
        // ref, create an Object::Value and push its object ref.
        // Otherwise, just push the value
        if !param.constant && !matches!(value.value, Value::ObjectRef(_)) {
            let index = heap.add_object(Object::Value(value.value));

            let object_ref = Value::ObjectRef(index);
            default_stack.push(StackValueWrapper::new(object_ref));

            // add to variables_to_be_updated
            if let Some(came_from) = value.came_from {
                variables_to_be_updated.push((came_from, index));
            }
        } else {
            default_stack.push(value);
        }
    }

    let return_value =
        execute_chunk(chunk, heap, Some(function_index), &default_stack);

    // update variables
    for (came_from, index) in variables_to_be_updated {
        let new_value = match heap.get_object(index) {
            Object::Value(v) => v,
            _ => unreachable!(),
        };

        execute_store_with_value(
            heap,
            variables,
            &came_from.name,
            &came_from.accessors,
            new_value,
        );
    }

    stack.push(StackValueWrapper::new(return_value));
}
