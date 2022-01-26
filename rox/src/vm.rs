use crate::Chunk;
use crate::Compiler;
use crate::OpCode;
use crate::Scanner;
use crate::Stack;
use crate::Value;
use crate::DEBUG_MODE;
use crate::{InterpretError, InterpretOutcome, InterpretResult};
use std::cell::RefCell;

pub struct VM {
    pub chunk: Chunk,
    ip: RefCell<usize>,
    stack: RefCell<Stack>,
    scanner: Scanner,
    compiler: Compiler,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: RefCell::new(0),
            stack: RefCell::new(Stack::new()),
            scanner: Scanner::new(),
            compiler: Compiler::new(),
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

    fn run(&self) -> InterpretResult {
        loop {
            // grab current IP
            let current_ip = self.ip.borrow().clone();
            // increment the IP
            *self.ip.borrow_mut() += 1;
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
                println!("Stack: {}", *self.stack.borrow());
                Chunk::disassemble_instruction(&instruction, current_ip, &self.chunk);
            }

            match instruction {
                OpCode::OpReturn(_) => {
                    println!("Popped: {}", self.stack.borrow_mut().pop());
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
                    self.stack.borrow_mut().push(constant);
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpNegate => {
                    let val = self.stack.borrow_mut().pop();
                    self.stack.borrow_mut().push(-val);
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpAdd => {
                    let b = self.stack.borrow_mut().pop(); // rhs operand
                    let a = self.stack.borrow_mut().pop(); // lhs operand
                    self.stack.borrow_mut().push(a + b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpSubtract => {
                    let b = self.stack.borrow_mut().pop(); // rhs operand
                    let a = self.stack.borrow_mut().pop(); // lhs operand
                    self.stack.borrow_mut().push(a - b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpMultiply => {
                    let b = self.stack.borrow_mut().pop(); // rhs operand
                    let a = self.stack.borrow_mut().pop(); // lhs operand
                    self.stack.borrow_mut().push(a * b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
                OpCode::OpDivide => {
                    let b = self.stack.borrow_mut().pop(); // rhs operand
                    let a = self.stack.borrow_mut().pop(); // lhs operand
                    self.stack.borrow_mut().push(a / b); // push result
                    return Ok(InterpretOutcome::InterpretOk);
                }
            }
        }
    }

    pub fn interpret(&self, source: &str) -> InterpretResult {
        for token in self.scanner.scan_tokens(source).iter() {
            self.compiler.compile(&token);
        }
        self.run()
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use super::*;
    use crate::RoxNumber;
    #[test]
    fn test_negate_op() {
        let mut vm = VM::new();
        vm.chunk.add_constant(Value::Number(RoxNumber(45.0)), 1);
        vm.chunk.write_chunk(OpCode::OpNegate, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(0), 1);

        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values.len(), 1);
        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values.len(), 1);
        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values.len(), 0);
    }

    #[test]
    fn test_add_binary_op() {
        let mut vm = VM::new();
        vm.chunk.add_constant(Value::Number(RoxNumber(45.0)), 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(15.0)), 1);
        vm.chunk.write_chunk(OpCode::OpAdd, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(12), 1);

        vm.interpret(&"".to_string()).unwrap();
        vm.interpret(&"".to_string()).unwrap();
        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values[0], Value::Number(RoxNumber(60.0)));
    }

    #[test]
    fn test_mult_op_1() {
        let mut vm = VM::new();
        vm.chunk.add_constant(Value::Number(RoxNumber(1.0)), 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(2.0)), 1);
        vm.chunk.write_chunk(OpCode::OpMultiply, 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(3.0)), 1);
        vm.chunk.write_chunk(OpCode::OpAdd, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.code.len() - 1 {
            vm.interpret(&"".to_string()).unwrap();
        }

        assert_eq!(vm.stack.borrow().values[0], Value::Number(RoxNumber(5.0)));
    }

    /// Test 1 + 2 * 3 == 7
    #[test]
    fn test_mult_op_2() {
        let mut vm = VM::new();
        vm.chunk.add_constant(Value::Number(RoxNumber(1.0)), 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(2.0)), 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(3.0)), 1);

        vm.chunk.write_chunk(OpCode::OpMultiply, 1);
        vm.chunk.write_chunk(OpCode::OpAdd, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.code.len() - 1 {
            vm.interpret(&"".to_string()).unwrap();
        }

        assert_eq!(vm.stack.borrow().values[0], Value::Number(RoxNumber(7.0)));
    }

    /// Test 3 - 2 - 1 == 0
    #[test]
    fn test_sub() {
        let mut vm = VM::new();
        vm.chunk.add_constant(Value::Number(RoxNumber(3.0)), 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(2.0)), 1);
        vm.chunk.write_chunk(OpCode::OpSubtract, 1);
        vm.chunk.add_constant(Value::Number(RoxNumber(1.0)), 1);
        vm.chunk.write_chunk(OpCode::OpSubtract, 1);
        vm.chunk.write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.code.len() - 1 {
            vm.interpret(&"".to_string()).unwrap();
        }

        assert_eq!(vm.stack.borrow().values[0], Value::Number(RoxNumber(0.0)));
    }

    #[test]
    fn test_order_operations() {
        let mut vm = VM::new();
        vm.chunk.add_constant(Value::Number(RoxNumber(1.0)), 1);
    }
}
