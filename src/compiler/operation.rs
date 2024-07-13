use super::value::Value;

#[derive(Debug, Clone)]
pub enum Operation {
    // push value to the stack
    Push(Value),
    // pop 1 value from the stack
    Pop,

    // pop 1 value from the stack and store it in a variable with the given
    // name
    Store(String),
    // load value of the varable with the given name and push it to the stack
    Load(String),

    // pop 2 values from the stack, add them and push that to the stack
    Add,
    // pop 2 values from the stack, substract them and push that to the stack
    Substract,
    // pop 2 values from the stack, multiply them and push that to the stack
    Multiply,
    // pop 2 values from the stack, divide them and push that to the stack
    Divide,
    // pop 1 value from the stack, negate it and push that to the stack
    Negate,

    // pop 2 values from the stack and push a boolean value saying whether
    // they are equal
    Equal,
    // pop 2 values from the stack and push a boolean value saying whether
    // the first one is less than the other
    LessThan,
    // pop 2 values from the stack and push a boolean value saying whether
    // the first one is less than or equal to the other
    LessThanOrEqual,
    // pop 2 values from the stack and push a boolean value saying whether
    // the first one is greater than the other
    GreaterThan,
    // pop 2 values from the stack and push a boolean value saying whether
    // the first one is greater than or equal to the other
    GreaterThanOrEqual,

    // jump to the given address in the code
    Jump(usize),
    // pop 1 value from the stack and jump to the given address in the code if
    // the value is true
    JumpIfFalse(usize),

    // Pop 1 Value::Function and N values for each function parameter, then
    // call the function
    Call,

    // halt execution
    Halt,
}
