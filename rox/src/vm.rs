use crate::Chunk;
use crate::OpCode;
use crate::Value;

pub struct VM {
    pub chunk: Chunk,
    ip: usize,
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
            ip: 0,
        }
    }

    fn read_byte(&mut self) -> Option<&OpCode> {
        if let Some(val) = self.chunk.code.get(self.ip) {
            self.ip += 1;
            Some(val)
        } else {
            None
        }
    }

    fn read_constant(&mut self) -> Option<Value> {
        None
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let current_ip = self.ip;
            let instruction = self.read_byte().expect(
                format!(
                    "Bytecode at IP {} did not return expected value!",
                    current_ip
                )
                .as_str(),
            );

            match instruction {
                OpCode::OpReturn(_) => return InterpretResult::InterpretOk,
                OpCode::OpConstant(_) => {
                    let current_ip = self.ip;
                    let constant = self.read_constant().expect(
                        format!(
                            "Constant at IP {} did not return expected value!",
                            current_ip
                        )
                        .as_str(),
                    );
                    println!("{}", constant);
                } //_ => return InterpretResult::InterpretCompileError,
            }
        }
    }
    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }
}
