use crate::parser::expression::VariableAccessor;

use super::value::{Object, Value};

#[derive(Debug, Clone)]
pub enum Operation {
    // push value to the stack
    Push(Value),
    // pop 1 value from the stack
    Pop,

    // push object to the stack, this will create the object in the heap and
    // push a Value::ObjectRef to the stack
    PushObject(Object),

    // Pop 1 value from the stack and store it in a variable with the given
    // name (either creates the variable or overwrites its value).
    // If accessors is not empty, they'll be traced one by one and the value
    // will be stored in the final accessor's location (this of course only
    // works if the variable already exists and is of the appropriate type
    // that contains all the right accessors - this is ensured by the
    // analyzer, same as everything else)
    Store {
        name: String,
        accessors: Vec<VariableAccessor>,
    },
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

    // Pop all struct fields from the stack and push a Value::Struct. The
    // given number is the number of fields in the struct.
    // Example:
    // - Push Value::StructFieldName("field1")
    // - Push Value::Int(123)
    // - Push Value::StructFieldName("field2")
    // - Push Value::Int(456)
    // - Push Value::StructFieldName("field3")
    // - Push Value::Int(789)
    // - MakeStruct(3)
    // This will pop all the 6 values that have been pushed to the stack and
    // push the following value:
    //   Value::Struct(HashMap::from([
    //       ("field1", Value::Int(123)),
    //       ("field2", Value::Int(456)),
    //       ("field3", Value::Int(789))
    //   ]))
    MakeStruct(usize),

    // Pop Value::Struct from stack and push the Value of the given field
    AccessField(String),

    // halt execution
    Halt,
}
