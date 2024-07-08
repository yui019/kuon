#[derive(Debug, Clone)]
pub enum Value {
    Null,
    String(String),
    Char(char),
    Int(i64),
    Float(f64),
}
