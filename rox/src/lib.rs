mod chunk;
mod error;
mod opcode;
mod run;
mod stack;
mod value;
mod vm;

pub use chunk::*;
pub use error::*;
pub use opcode::OpCode;
pub use run::*;
pub use stack::*;
pub use value::*;
pub use vm::*;

pub static DEBUG_MODE: bool = true;
pub const STACK_MAX: i32 = 256;
