/**
 * The OpCodes for the Chunk instructions
 * to be used for the Rox VM
 */
#[derive(Debug)]
pub enum OpCode {
    OpReturn(usize),
    OpConstant(usize), // the internal value is treated as index into constant values array
}

impl std::clone::Clone for OpCode {
    fn clone(&self) -> Self {
        match self {
            OpCode::OpReturn(val) => OpCode::OpReturn(val.clone()),
            OpCode::OpConstant(val) => OpCode::OpConstant(val.clone())
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::OpReturn(_) => write!(f, "OP_RETURN"),
            OpCode::OpConstant(_) => write!(f, "OP_CONSTANT"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    offset: usize,
}
