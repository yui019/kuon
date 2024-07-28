use crate::compiler::value::Value;

use super::StackValueWrapper;

pub fn add(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Int(a + b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(*a as f64 + b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a + *b as f64),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a + b),
            came_from: None,
        },

        _ => panic!("Can only add numbers together"),
    }
}

pub fn substract(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Int(a - b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(*a as f64 - b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a - *b as f64),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a - b),
            came_from: None,
        },

        _ => panic!("Can only substract numbers together"),
    }
}

pub fn multiply(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Int(a * b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(*a as f64 * b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a * *b as f64),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a * b),
            came_from: None,
        },

        _ => panic!("Can only multiply numbers together"),
    }
}

pub fn divide(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Int(a / b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(*a as f64 / b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a / *b as f64),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Float(a / b),
            came_from: None,
        },

        _ => panic!("Can only divide numbers together"),
    }
}

pub fn negate(value: &StackValueWrapper) -> StackValueWrapper {
    match value {
        StackValueWrapper {
            value: Value::Int(a),
            ..
        } => StackValueWrapper {
            value: Value::Int(-a),
            came_from: None,
        },

        StackValueWrapper {
            value: Value::Float(a),
            ..
        } => StackValueWrapper {
            value: Value::Float(-a),
            came_from: None,
        },

        StackValueWrapper {
            value: Value::Bool(b),
            ..
        } => StackValueWrapper {
            value: Value::Bool(!b),
            came_from: None,
        },

        _ => unreachable!(),
    }
}

pub fn equal(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    // TODO: handle objects specially here
    // For now it just compares the references inside of them (this isn't C...)
    StackValueWrapper {
        value: Value::Bool(first.value == second.value),
        came_from: None,
    }
}

pub fn less_than(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a < b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool((*a as f64) < *b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(*a < (*b as f64)),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a < b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn less_than_or_equal(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a <= b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool((*a as f64) <= *b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(*a <= (*b as f64)),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a <= b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn greater_than(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a > b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool((*a as f64) > *b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(*a > (*b as f64)),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a > b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn greater_than_or_equal(
    first: &StackValueWrapper,
    second: &StackValueWrapper,
) -> StackValueWrapper {
    match (first, second) {
        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a >= b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Int(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool((*a as f64) >= *b),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Int(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(*a >= (*b as f64)),
            came_from: None,
        },

        (
            StackValueWrapper {
                value: Value::Float(a),
                ..
            },
            StackValueWrapper {
                value: Value::Float(b),
                ..
            },
        ) => StackValueWrapper {
            value: Value::Bool(a >= b),
            came_from: None,
        },

        _ => panic!("Can only compare numbers"),
    }
}

pub fn is_true(value: &StackValueWrapper) -> bool {
    match value {
        StackValueWrapper {
            value: Value::Bool(b),
            ..
        } => *b,

        _ => panic!("Can't determine if non-boolean value is true"),
    }
}
