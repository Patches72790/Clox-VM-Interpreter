/**
 * The OpCodes for the Chunk instructions
 * to be used for the Rox VM
 */
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    OpReturn(usize),
    OpConstant(usize), // the internal value is treated as index into constant values array
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpPrint,
    OpPop,
    OpDefineGlobal(usize), // stores the index of the string identifier in the constants array
    OpGetGlobal(usize),
    OpSetGlobal(usize),
    OpGetLocal(usize),
    OpSetLocal(usize),
    OpJumpIfFalse(Option<usize>),
    OpJump(Option<usize>),
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::OpReturn(_) => write!(f, "OP_RETURN"),
            OpCode::OpConstant(_) => write!(f, "OP_CONSTANT"),
            OpCode::OpNegate => write!(f, "OP_NEGATE"),
            OpCode::OpAdd => write!(f, "OP_ADD"),
            OpCode::OpSubtract => write!(f, "OP_SUBTRACT"),
            OpCode::OpMultiply => write!(f, "OP_MULTIPLY"),
            OpCode::OpDivide => write!(f, "OP_DIVIDE"),
            OpCode::OpNil => write!(f, "OP_NIL"),
            OpCode::OpTrue => write!(f, "OP_TRUE"),
            OpCode::OpFalse => write!(f, "OP_FALSE"),
            OpCode::OpNot => write!(f, "OP_NOT"),
            OpCode::OpEqual => write!(f, "OP_EQUAL"),
            OpCode::OpGreater => write!(f, "OP_GREATER"),
            OpCode::OpLess => write!(f, "OP_LESS"),
            OpCode::OpPrint => write!(f, "OP_PRINT"),
            OpCode::OpPop => write!(f, "OP_POP"),
            OpCode::OpDefineGlobal(_) => write!(f, "OP_DEFINE_GLOBAL"),
            OpCode::OpGetGlobal(_) => write!(f, "OP_GET_GLOBAL"),
            OpCode::OpSetGlobal(_) => write!(f, "OP_SET_GLOBAL"),
            OpCode::OpSetLocal(_) => write!(f, "OP_SET_LOCAL"),
            OpCode::OpGetLocal(_) => write!(f, "OP_GET_LOCAL"),
            OpCode::OpJumpIfFalse(_) => write!(f, "OP_JUMP_IF_FALSE"),
            OpCode::OpJump(_) => write!(f, "OP_JUMP"),
        }
    }
}

pub enum VariableOp {
    GetGlobal,
    SetGlobal,
    Define,
}
