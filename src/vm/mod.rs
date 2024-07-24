use std::collections::HashMap;

use util::{
    add, divide, equal, greater_than, greater_than_or_equal, is_true,
    less_than, less_than_or_equal, multiply, negate, substract,
};

use crate::compiler::{chunk::Chunk, operation::Operation, value::Value};

mod util;

pub fn execute(chunk: &Chunk) -> Value {
    execute_chunk(chunk, None, &vec![])
}

/// If function is Some, the function with that index in the chunk will be
/// executed instead of the chunk
fn execute_chunk(
    chunk: &Chunk,
    function: Option<usize>,
    default_stack: &Vec<Value>,
) -> Value {
    let mut stack: Vec<Value> = vec![];
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
                stack.push(v);
            }
            Operation::Pop => {
                stack.pop();
            }

            Operation::Store(name) => {
                let value = stack.pop();
                variables.insert(name, value.unwrap());
            }
            Operation::Load(name) => {
                let value = variables[&name].clone();
                stack.push(value);
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
                    Some(Value::Function(index)) => index,
                    _ => unreachable!(),
                };
                let function = &chunk.functions[function_index];

                let mut default_stack: Vec<Value> = vec![];
                for _ in 0..function.parameter_count {
                    default_stack.push(stack.pop().unwrap());
                }

                let return_value =
                    execute_chunk(chunk, Some(function_index), &default_stack);

                stack.push(return_value);
            }

            Operation::MakeStruct(field_count) => {
                let mut fields: HashMap<String, Value> = HashMap::new();
                for _ in 0..field_count {
                    let value = match stack.pop() {
                        Some(v) => v,
                        None => panic!("Expected Value"),
                    };

                    let name = match stack.pop() {
                        Some(Value::StructFieldName(s)) => s,

                        _ => panic!(
                            "Expected a Value::StructFieldName with the field name"
                        ),
                    };

                    fields.insert(name, value);
                }

                stack.push(Value::Struct(fields));
            }

            Operation::AccessField(name) => {
                let fields = match stack.pop() {
                    Some(Value::Struct(f)) => f,

                    _ => panic!("Expected Value::Struct"),
                };

                let value = match fields.get(&name) {
                    Some(v) => v,

                    None => panic!("Field {} does not exist", name),
                };

                stack.push(value.clone());
            }

            Operation::Halt => return stack.pop().unwrap_or(Value::Null),
        }

        i += 1;
    }

    Value::Null
}
