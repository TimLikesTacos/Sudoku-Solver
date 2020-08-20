//! Custom errors
#[derive(Debug)]
pub enum SudError {
    InputParse,
    OutputParse,
}

impl std::error::Error for SudError {}

impl std::fmt::Display for SudError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudError::InputParse => write!(f, "Parse error when attempting puzzle input"),
            SudError::OutputParse => write!(f, "Parse error when attempting output"),
        }
    }
}
