use crate::compiler::value::Value;

pub fn add(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 + b),
        (Value::Float(a), Value::Int(b)) => Value::Float(a + *b as f64),
        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),

        _ => panic!("Can only add numbers together"),
    }
}

pub fn substract(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 - b),
        (Value::Float(a), Value::Int(b)) => Value::Float(a - *b as f64),
        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),

        _ => panic!("Can only substract numbers together"),
    }
}

pub fn multiply(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 * b),
        (Value::Float(a), Value::Int(b)) => Value::Float(a * *b as f64),
        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),

        _ => panic!("Can only multiply numbers together"),
    }
}

pub fn divide(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
        (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 / b),
        (Value::Float(a), Value::Int(b)) => Value::Float(a / *b as f64),
        (Value::Float(a), Value::Float(b)) => Value::Float(a / b),

        _ => panic!("Can only divide numbers together"),
    }
}

pub fn negate(value: &Value) -> Value {
    match value {
        Value::Int(a) => Value::Int(-a),
        Value::Float(a) => Value::Float(-a),
        Value::Bool(b) => Value::Bool(!b),

        _ => unreachable!(),
    }
}

pub fn equal(first: &Value, second: &Value) -> Value {
    Value::Bool(first == second)
}

pub fn less_than(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
        (Value::Int(a), Value::Float(b)) => Value::Bool((*a as f64) < *b),
        (Value::Float(a), Value::Int(b)) => Value::Bool(*a < (*b as f64)),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),

        _ => panic!("Can only compare numbers"),
    }
}

pub fn less_than_or_equal(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
        (Value::Int(a), Value::Float(b)) => Value::Bool((*a as f64) <= *b),
        (Value::Float(a), Value::Int(b)) => Value::Bool(*a <= (*b as f64)),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),

        _ => panic!("Can only compare numbers"),
    }
}

pub fn greater_than(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
        (Value::Int(a), Value::Float(b)) => Value::Bool((*a as f64) > *b),
        (Value::Float(a), Value::Int(b)) => Value::Bool(*a > (*b as f64)),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),

        _ => panic!("Can only compare numbers"),
    }
}

pub fn greater_than_or_equal(first: &Value, second: &Value) -> Value {
    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
        (Value::Int(a), Value::Float(b)) => Value::Bool((*a as f64) >= *b),
        (Value::Float(a), Value::Int(b)) => Value::Bool(*a >= (*b as f64)),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),

        _ => panic!("Can only compare numbers"),
    }
}

pub fn is_true(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,

        _ => panic!("Can't determine if non-boolean value is true"),
    }
}
