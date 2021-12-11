pub type InterpretResult = std::result::Result<InterpretOutcome, InterpretError>;

pub enum InterpretOutcome {
InterpretOk,
InterpretCompileError(InterpretError),
InterpretRuntimeError(InterpretError),
}

#[derive(Debug, Clone)]
pub struct InterpretError {
    message: String,
}

impl InterpretError {
    pub fn new(msg: &str) -> InterpretError {
        InterpretError {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
