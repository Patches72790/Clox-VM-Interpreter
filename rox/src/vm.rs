use crate::Chunk;

pub struct VM {
    pub chunk: Chunk,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
        }
    }

    
    pub fn interpret() -> InterpretResult {
        unimplemented!()
    }
}
