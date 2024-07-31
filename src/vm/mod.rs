use std::collections::HashMap;

use execution_result::ExecutionResult;
use heap::VmHeap;
use operations::{call::execute_call, store::execute_store};
use stack_value_wrapper::StackValueWrapper;
use util::{
    add, divide, equal, greater_than, greater_than_or_equal, is_true,
    less_than, less_than_or_equal, multiply, negate, substract,
};

use crate::{
    compiler::{
        chunk::Chunk,
        operation::Operation,
        value::{Object, Value},
    },
    parser::expression::VariableAccessor,
};

pub mod execution_result;
mod heap;
mod operations;
pub mod stack_value_wrapper;
mod util;

pub fn execute(chunk: &Chunk) -> ExecutionResult {
    let mut heap = VmHeap::new();

    let value = execute_chunk(chunk, &mut heap, None, &vec![]);

    ExecutionResult::from_value(chunk, &heap, value)
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

            Operation::Store { name, accessors } => {
                execute_store(
                    heap,
                    &mut stack,
                    &mut variables,
                    &name,
                    &accessors,
                );
            }
            Operation::Load(name) => {
                let value = variables[&name].clone();

                stack.push(StackValueWrapper::new_from_name(value, name));
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
                execute_call(chunk, heap, &mut stack, &mut variables);
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
                let value = stack.pop().unwrap();

                let fields = match value {
                    StackValueWrapper {
                        value: Value::Struct(f),
                        ..
                    } => f,

                    StackValueWrapper {
                        value: Value::ObjectRef(index),
                        ..
                    } => {
                        let object = heap.get_object(index);
                        match object {
                            Object::Value(Value::Struct(f)) => f,

                            _ => panic!("Expected Value::Struct"),
                        }
                    }

                    _ => panic!("Expected Value::Struct"),
                };

                let field_value = match fields.get(&name) {
                    Some(v) => v,

                    None => panic!("Field {} does not exist", name),
                };

                match value {
                    StackValueWrapper {
                        came_from: Some(came_from),
                        ..
                    } => {
                        let mut accessors = came_from.accessors;
                        accessors.push(VariableAccessor::StructField(name));

                        stack.push(
                            StackValueWrapper::new_from_name_and_accessors(
                                field_value.clone(),
                                came_from.name,
                                accessors,
                            ),
                        );
                    }

                    _ => {
                        stack.push(StackValueWrapper::new(field_value.clone()));
                    }
                }
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
