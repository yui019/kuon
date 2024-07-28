use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),

    // this is for when you want to pass a type like an int by reference in a
    // function
    Value(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Char(char),
    Int(i64),
    Float(f64),
    Bool(bool),

    // this is a reference to an object that's stored in the VM's heap
    ObjectRef(usize),

    // the usize parameter is an index into the functions Vec field of the
    // Chunk struct
    Function(usize),

    Struct(HashMap<String, Value>),

    // uhhh the reason I need this is that the way my bytecode compiler works
    // for now is that I call a function on an expression and that function
    // adds a Push operation to the chunk, meaning I can't just extract the
    // Value, it's added directly to the code instead (no, I will not refactor
    // it). So basically the way I handle pushing structs is I push a string
    // with the field name, then compile the value of it so it's pushed neaxt
    // and repeat that process with the same field (so I get: push
    // string(field1), push value, push string(field2), push value, etc.). And
    // pushing regular String objects would be bad since they're garbage
    // collected, so I switched to this instead.
    // (please don't hate me, I promise I'll improve on this)
    StructFieldName(String),
}
