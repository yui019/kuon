use super::value::Value;

#[derive(Debug, Clone)]
pub enum Operation {
    Push(Value),
    Pop,

    Add,
    Substract,
    Multiply,
    Divide,
    Negate,

    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    Halt,
}
