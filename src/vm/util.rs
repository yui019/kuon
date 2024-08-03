use crate::compiler::value::{Object, Value};

use super::{heap::VmHeap, StackValueWrapper};

fn extract_value(heap: &VmHeap, value: &StackValueWrapper) -> Value {
    let mut value = value.value.clone();

    match value {
        Value::ObjectRef(index) => match heap.get_object(index) {
            Object::Value(v) => {
                let v = v.clone();
                value = v;
            }

            _ => {}
        },

        _ => {}
    }

    value
}

pub fn add(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Int(a + b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a as f64 + b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Float(a + b as f64),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a + b),
            came_from: None,
        },

        _ => panic!("Can only add numbers together"),
    }
}

pub fn substract(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Int(a - b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a as f64 - b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Float(a - b as f64),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a - b),
            came_from: None,
        },

        _ => panic!("Can only substract numbers together"),
    }
}

pub fn multiply(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Int(a * b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a as f64 * b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Float(a * b as f64),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a * b),
            came_from: None,
        },

        _ => panic!("Can only multiply numbers together"),
    }
}

pub fn divide(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Int(a / b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a as f64 / b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Float(a / b as f64),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Float(a / b),
            came_from: None,
        },

        _ => panic!("Can only divide numbers together"),
    }
}

pub fn negate(heap: &VmHeap, value: &StackValueWrapper) -> StackValueWrapper {
    let value = extract_value(heap, value);

    match value {
        Value::Int(a) => StackValueWrapper {
            value: Value::Int(-a),
            came_from: None,
        },

        Value::Float(a) => StackValueWrapper {
            value: Value::Float(-a),
            came_from: None,
        },

        Value::Bool(b) => StackValueWrapper {
            value: Value::Bool(!b),
            came_from: None,
        },

        _ => unreachable!(),
    }
}

pub fn equal(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = first.value.clone();
    let second = second.value.clone();

    let first_is_object = matches!(first, Value::ObjectRef(_));
    let second_is_object = matches!(first, Value::ObjectRef(_));

    let mut equal = false;

    if first_is_object && second_is_object {
        let first_index = match first {
            Value::ObjectRef(index) => index,
            _ => unreachable!(),
        };

        let second_index = match second {
            Value::ObjectRef(index) => index,
            _ => unreachable!(),
        };

        let first_object = heap.get_object(first_index);
        let second_object = heap.get_object(second_index);

        equal = first_object == second_object;
    }

    if !first_is_object && second_is_object {
        let second_index = match second {
            Value::ObjectRef(index) => index,
            _ => unreachable!(),
        };

        let second_object = heap.get_object(second_index);

        match second_object {
            Object::Value(second_value) => {
                equal = first == second_value;
            }

            _ => {
                equal = false;
            }
        }
    }

    if first_is_object && !second_is_object {
        let first_index = match first {
            Value::ObjectRef(index) => index,
            _ => unreachable!(),
        };

        let first_object = heap.get_object(first_index);

        match first_object {
            Object::Value(first_value) => {
                equal = first_value == second;
            }

            _ => {
                equal = false;
            }
        }
    }

    if !first_is_object && !second_is_object {
        equal = first == second;
    }

    StackValueWrapper {
        value: Value::Bool(equal),
        came_from: None,
    }
}

pub fn less_than(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a < b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool((a as f64) < b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a < (b as f64)),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool(a < b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn less_than_or_equal(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a <= b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool((a as f64) <= b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a <= (b as f64)),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool(a <= b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn greater_than(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a > b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool((a as f64) > b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a > (b as f64)),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool(a > b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn greater_than_or_equal(
    heap: &VmHeap,
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    let first = extract_value(heap, first);
    let second = extract_value(heap, second);

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a >= b),
            came_from: None,
        },

        (Value::Int(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool((a as f64) >= b),
            came_from: None,
        },

        (Value::Float(a), Value::Int(b)) => StackValueWrapper {
            value: Value::Bool(a >= (b as f64)),
            came_from: None,
        },

        (Value::Float(a), Value::Float(b)) => StackValueWrapper {
            value: Value::Bool(a >= b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn is_true(heap: &VmHeap, value: &StackValueWrapper) -> bool {
    let value = extract_value(heap, value);

    match value {
        Value::Bool(b) => b,

        _ => panic!("Can't determine if non-boolean value is true"),
    }
}
