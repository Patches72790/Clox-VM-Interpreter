/**
 * The OpCodes for the Chunk instructions
 * to be used for the Rox VM
 */
#[derive(Debug)]
pub enum OpCode {
    OpReturn(usize),
    OpConstant(usize), // the internal value is treated as index into constant values array
}

#[derive(Debug)]
struct Instruction {
    op_code: OpCode,
    offset: usize,
}
