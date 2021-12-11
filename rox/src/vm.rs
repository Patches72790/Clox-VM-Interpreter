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
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpNegate => {
                    let val = self.stack.pop();
                    self.stack.push(-val);
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpAdd => {
                    let b = self.stack.pop(); // rhs operand
                    let a = self.stack.pop(); // lhs operand
                    self.stack.push(a + b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpSubtract => {
                    let b = self.stack.pop(); // rhs operand
                    let a = self.stack.pop(); // lhs operand
                    self.stack.push(a - b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpMultiply => {
                    let b = self.stack.pop(); // rhs operand
                    let a = self.stack.pop(); // lhs operand
                    self.stack.push(a * b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpDivide => {
                    let b = self.stack.pop(); // rhs operand
                    let a = self.stack.pop(); // lhs operand
                    self.stack.push(a / b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
            }
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use super::*;
    #[test]
    fn test_negate_op() {
        let mut vm = VM::new();
        let index = vm.chunk.add_constant(Value::Number(45.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        vm.chunk.write_chunk(OpCode::OpNegate, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(0), 1);

        vm.interpret().unwrap();
        assert_eq!(vm.stack.values.len(), 1);
        vm.interpret().unwrap();
        assert_eq!(vm.stack.values.len(), 1);
        vm.interpret().unwrap();
        assert_eq!(vm.stack.values.len(), 0);
    }

    #[test]
    fn test_add_binary_op() {
        let mut vm = VM::new();
        let index = vm.chunk.add_constant(Value::Number(45.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        let index = vm.chunk.add_constant(Value::Number(15.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        vm.chunk.write_chunk(OpCode::OpAdd, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(12), 1);

        vm.interpret().unwrap();
        vm.interpret().unwrap();
        vm.interpret().unwrap();
        assert_eq!(vm.stack.values[0], Value::Number(60.0));
    }

    #[test]
    fn test_mult_op_1() {
        let mut vm = VM::new();
        let index = vm.chunk.add_constant(Value::Number(1.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        let index = vm.chunk.add_constant(Value::Number(2.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        vm.chunk.write_chunk(OpCode::OpMultiply, 1);
        let index = vm.chunk.add_constant(Value::Number(3.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        vm.chunk.write_chunk(OpCode::OpAdd, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.code.len() - 1 {
            vm.interpret().unwrap();
        }

        assert_eq!(vm.stack.values[0], Value::Number(5.0));
    }

    /// Test 1 + 2 * 3 == 7
    #[test]
    fn test_mult_op_2() {
        let mut vm = VM::new();
        let index = vm.chunk.add_constant(Value::Number(1.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        let index = vm.chunk.add_constant(Value::Number(2.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        let index = vm.chunk.add_constant(Value::Number(3.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);

        vm.chunk.write_chunk(OpCode::OpMultiply, 1);
        vm.chunk.write_chunk(OpCode::OpAdd, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.code.len() - 1 {
            vm.interpret().unwrap();
        }

        assert_eq!(vm.stack.values[0], Value::Number(7.0));
    }

    /// Test 3 - 2 - 1 == 0
    #[test]
    fn test_sub() {
        let mut vm = VM::new();
        let index = vm.chunk.add_constant(Value::Number(3.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        let index = vm.chunk.add_constant(Value::Number(2.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        vm.chunk.write_chunk(OpCode::OpSubtract, 1);
        let index = vm.chunk.add_constant(Value::Number(1.0));
        vm.chunk.write_chunk(OpCode::OpConstant(index), 1);
        vm.chunk.write_chunk(OpCode::OpSubtract, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.code.len() - 1 {
            vm.interpret().unwrap();
        }

        assert_eq!(vm.stack.values[0], Value::Number(0.0));
    }
}
