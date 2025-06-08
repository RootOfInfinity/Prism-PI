#[derive(Clone, Debug)]
pub struct CompileError {
    pub e_type: ErrorType,
    pub line: u32,
    pub col: u32,
}
impl CompileError {
    pub fn new(e_type: ErrorType, line: u32, col: u32) -> Self {
        CompileError { e_type, line, col }
    }
}
#[derive(Clone, Debug)]
pub enum ErrorType {
    LexingError,
    ParsingError,
    TypeError,
}
