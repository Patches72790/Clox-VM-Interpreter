use crate::Chunk;
use crate::OpCode;
use crate::Stack;
use crate::Value;
use crate::DEBUG_MODE;
use crate::{InterpretError, InterpretOutcome, InterpretResult};

pub struct VM {
    pub chunk: Chunk,
    ip: usize,
    stack: Stack,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Stack::new(),
        }
    }

    fn read_byte(code: &Vec<OpCode>, ip: usize) -> Option<OpCode> {
        if let Some(val) = code.get(ip) {
            Some(*val)
        } else {
            None
        }
    }

    fn read_constant(values: &Vec<Value>, index: usize) -> Option<Value> {
        if let Some(val) = values.get(index) {
            Some(*val)
        } else {
            None
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let current_ip = self.ip;
            self.ip += 1;
            let instruction = match VM::read_byte(&self.chunk.code, current_ip) {
                Some(instr) => instr,
                None => {
                    return Err(InterpretError::new(
                        format!(
                            "Bytecode at IP {} did not return expected value!",
                            current_ip
                        )
                        .as_str(),
                    ))
                }
            };

            if DEBUG_MODE {
                println!("Stack: {}", self.stack);
                Chunk::disassemble_instruction(&instruction, current_ip, &self.chunk);
            }

            match instruction {
                OpCode::OpReturn(_) => {
                    println!("Popped: {}", self.stack.pop());
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpConstant(constants_index) => {
                    let constant = VM::read_constant(&self.chunk.constants.values, constants_index)
                        .expect(
                            format!(
                                "Constant at IP {} did not return expected value!",
                                current_ip
                            )
                            .as_str(),
                        );
                    self.stack.push(constant);
                    break Ok(InterpretOutcome::InterpretOk);
                }
            }
        }
    }
    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }
}
