pub type InterpretResult = std::result::Result<InterpretOk, InterpretError>;

pub struct InterpretOk;

#[derive(Debug, Clone)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String),
}

impl From<&str> for InterpretError {
    fn from(msg: &str) -> Self {
        InterpretError::RuntimeError(msg.to_string())
    }
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::CompileError(message) => write!(f, "{}", message),
            InterpretError::RuntimeError(message) => write!(f, "{}", message),
        }
    }
}
