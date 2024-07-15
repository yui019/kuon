#[derive(Debug)]
pub struct AnalyzerError {
    pub line: usize,

    pub message: String,
}

#[macro_export]
macro_rules! analyzer_error {
    ($line:expr, $($arg:tt)*) => {{
        Err($crate::analyzer::analyzer_error::AnalyzerError::new($line as usize, format!($($arg)*)))
    }}
}

impl AnalyzerError {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}
