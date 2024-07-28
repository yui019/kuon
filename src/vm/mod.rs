use std::collections::HashMap;

use heap::VmHeap;
use util::{
    add, divide, equal, greater_than, greater_than_or_equal, is_true,
    less_than, less_than_or_equal, multiply, negate, substract,
};

use crate::compiler::{
    chunk::Chunk,
    operation::Operation,
    value::{Object, Value},
};

mod heap;
mod util;

#[derive(Debug, Clone, PartialEq)]
struct StackValueWrapper {
    value: Value,
    came_from: Option<String>,
}

impl StackValueWrapper {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            came_from: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecuteResult {
    Value(Value),
    Object(Object),
}

pub fn execute(chunk: &Chunk) -> ExecuteResult {
    let mut heap = VmHeap::new();

    let value = execute_chunk(chunk, &mut heap, None, &vec![]);
    println!("{:?}", heap);

    match value {
        Value::ObjectRef(index) => {
            ExecuteResult::Object(heap.get_object(index))
        }

        v => ExecuteResult::Value(v),
    }
}

/// If function is Some, the function with that index in the chunk will be
/// executed instead of the chunk
fn execute_chunk(
    chunk: &Chunk,
    heap: &mut VmHeap,
    function: Option<usize>,
    default_stack: &Vec<StackValueWrapper>,
) -> Value {
    let mut stack: Vec<StackValueWrapper> = vec![];
    stack.append(&mut default_stack.clone());
    let mut variables: HashMap<String, Value> = HashMap::new();

    let code = match function {
        Some(index) => &chunk.functions[index].chunk.code,
        None => &chunk.code,
    };

    let mut i = 0;
    loop {
        if i == code.len() {
            break;
        }

        let operation = &code[i];

        match operation.clone() {
            Operation::Push(v) => {
                stack.push(StackValueWrapper::new(v));
            }
            Operation::Pop => {
                stack.pop();
            }

            Operation::PushObject(o) => {
                let index = heap.add_object(o);
                let object_ref = Value::ObjectRef(index);
                stack.push(StackValueWrapper::new(object_ref));
            }

            Operation::Store(name) => {
                let value = stack.pop().unwrap();

                // if you're storing into a variable which already holds an
                // object ref, then update the object it's referencing with that
                // value
                if let Some(Value::ObjectRef(dest_index)) = variables.get(&name)
                {
                    let object = match &value {
                        // if you're storing an object ref, get the object from
                        // the heap
                        StackValueWrapper {
                            value: Value::ObjectRef(src_index),
                            ..
                        } => heap.get_object(*src_index),

                        // otherwise, create an Object::Value
                        v => Object::Value(v.clone().value),
                    };

                    heap.update_object(*dest_index, object);
                }

                variables.insert(name, value.value);
            }
            Operation::Load(name) => {
                let value = variables[&name].clone();

                stack.push(StackValueWrapper {
                    value,
                    came_from: Some(name),
                });
            }

            Operation::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(add(&a, &b));
            }
            Operation::Substract => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(substract(&a, &b));
            }
            Operation::Multiply => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(multiply(&a, &b));
            }
            Operation::Divide => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(divide(&a, &b));
            }
            Operation::Negate => {
                let value = stack.pop().unwrap();
                stack.push(negate(&value));
            }
            Operation::Equal => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(equal(&a, &b));
            }
            Operation::LessThan => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(less_than(&a, &b));
            }
            Operation::LessThanOrEqual => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(less_than_or_equal(&a, &b));
            }
            Operation::GreaterThan => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(greater_than(&a, &b));
            }
            Operation::GreaterThanOrEqual => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(greater_than_or_equal(&a, &b));
            }

            Operation::Jump(address) => {
                i = address;
                continue;
            }
            Operation::JumpIfFalse(address) => {
                if !is_true(&stack.pop().unwrap()) {
                    i = address;
                    continue;
                }
            }

            Operation::Call => {
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

                // array of bindings between variable names and object ref
                // indexes. When the function has finished executing, the
                // variables with those names will be updated to the values
                // inside those objects
                let mut variables_to_be_updated: Vec<(String, usize)> = vec![];

                // pop variables from stack into default_stack
                for param in &function.parameters {
                    let value = stack.pop().unwrap();

                    // if the parameter is not a constant (has a var in front in
                    // the function definition) and it's not already an object
                    // ref, create an Object::Value and push its object ref.
                    // Otherwise, just push the value
                    if !param.constant
                        && !matches!(value.value, Value::ObjectRef(_))
                    {
                        let index = heap.add_object(Object::Value(value.value));

                        let object_ref = Value::ObjectRef(index);
                        default_stack.push(StackValueWrapper::new(object_ref));

                        // add to variables_to_be_updated
                        if let Some(name) = value.came_from {
                            variables_to_be_updated.push((name, index));
                        }
                    } else {
                        default_stack.push(value);
                    }
                }

                let return_value = execute_chunk(
                    chunk,
                    heap,
                    Some(function_index),
                    &default_stack,
                );

                // update variables
                for (name, index) in variables_to_be_updated {
                    let new_value = match heap.get_object(index) {
                        Object::Value(v) => v,
                        _ => unreachable!(),
                    };

                    // insert updates if the entry already exists (which it
                    // always does in this case btw)
                    variables.insert(name, new_value);
                }

                stack.push(StackValueWrapper::new(return_value));
            }

            Operation::MakeStruct(field_count) => {
                let mut fields: HashMap<String, Value> = HashMap::new();
                for _ in 0..field_count {
                    let value = match stack.pop() {
                        Some(v) => v,
                        None => panic!("Expected Value"),
                    };

                    let name = match stack.pop() {
                        Some(StackValueWrapper{value:Value::StructFieldName(s),..}) => s,

                        _ => panic!(
                            "Expected a Value::StructFieldName with the field name"
                        ),
                    };

                    fields.insert(name, value.value);
                }

                stack.push(StackValueWrapper::new(Value::Struct(fields)));
            }

            Operation::AccessField(name) => {
                let fields = match stack.pop() {
                    Some(StackValueWrapper {
                        value: Value::Struct(f),
                        ..
                    }) => f,

                    _ => panic!("Expected Value::Struct"),
                };

                let value = match fields.get(&name) {
                    Some(v) => v,

                    None => panic!("Field {} does not exist", name),
                };

                stack.push(StackValueWrapper::new(value.clone()));
            }

            Operation::Halt => {
                return stack
                    .pop()
                    .unwrap_or(StackValueWrapper::new(Value::Null))
                    .value
            }
        }

        i += 1;
    }

    Value::Null
}
