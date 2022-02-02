mod chunk;
mod compiler;
mod error;
mod opcode;
mod parser;
mod precedence;
mod run;
mod scanner;
mod stack;
mod token;
mod value;
mod vm;

pub use chunk::*;
pub use compiler::*;
pub use error::*;
pub use opcode::OpCode;
pub use parser::*;
pub use precedence::Precedence;
pub use run::*;
pub use scanner::Scanner;
pub use stack::*;
pub use token::*;
pub use value::*;
pub use vm::*;

pub static DEBUG_MODE: bool = true;
pub const STACK_MAX: i32 = 256;
