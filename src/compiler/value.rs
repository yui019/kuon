#[derive(Debug, Clone)]
pub enum Value {
    Null,
    String(String),
    Char(char),
    Int(i64),
    Float(f64),

    // the usize parameter is an index into the functions Vec field of the
    // Chunk struct
    Function(usize),
}
