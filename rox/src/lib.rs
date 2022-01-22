mod chunk;
mod compiler;
mod error;
mod opcode;
mod run;
mod scanner;
mod stack;
mod value;
mod vm;
mod token;

pub use chunk::*;
pub use error::*;
pub use opcode::OpCode;
pub use run::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

pub static DEBUG_MODE: bool = true;
pub const STACK_MAX: i32 = 256;
