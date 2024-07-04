#[derive(Debug)]
pub struct ParserError {
    // -1 means EOF
    pub line: i32,

    pub message: String,
}

#[macro_export]
macro_rules! parser_error {
    ($line:expr, $($arg:tt)*) => {{
        $crate::parser::parser_error::ParserError::new($line as i32, format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! parser_error_eof {
    ($($arg:tt)*) => {{
        $crate::parser::parser_error::ParserError::new(-1, format!($($arg)*))
    }}
}

impl ParserError {
    pub fn new(line: i32, message: String) -> Self {
        Self { line, message }
    }
}
