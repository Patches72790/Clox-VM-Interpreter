use crate::Chunk;
use crate::Compiler;
use crate::ObjectType;
use crate::OpCode;
use crate::RoxObject;
use crate::RoxString;
use crate::Scanner;
use crate::Stack;
use crate::Value;
use crate::DEBUG_MODE;
use crate::{InterpretError, InterpretOk, InterpretResult};
use std::cell::RefCell;
use std::rc::Rc;

pub struct VM {
    pub chunk: Rc<RefCell<Chunk>>,
    ip: RefCell<usize>,
    stack: RefCell<Stack>,
    scanner: Scanner,
    objects: RefCell<Option<*mut RoxObject>>,
}

impl<'a> VM {
    pub fn new() -> VM {
        let chunk = Rc::new(RefCell::new(Chunk::new()));
        VM {
            chunk: Rc::clone(&chunk),
            ip: RefCell::new(0),
            stack: RefCell::new(Stack::new()),
            scanner: Scanner::new(),
            objects: RefCell::new(None),
        }
    }

    ///
    /// This adds an allocated object to the implicit linked list
    /// of objects tracked by this VM.
    ///
    pub fn add_object(&self, new_object: &mut RoxObject) {
        // prepend to linked list
        new_object.next_object = *self.objects.borrow();
        *self.objects.borrow_mut() = Some(new_object);
    }

    fn print_objects(&self) {
        unsafe {
            let mut current = *self.objects.borrow();
            while let Some(obj) = current {
                println!("{}", (*obj).object_type);
                current = (*obj).next_object;
            }
        }
    }

    pub fn reset(&mut self) {
        *(self.ip.borrow_mut()) = 0;
        self.chunk.borrow_mut().reset();
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
            Some(val.clone())
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

            // read next instruction
            let instruction = match VM::read_byte(&self.chunk.borrow().code, current_ip) {
                Some(instr) => instr,
                None => {
                    if DEBUG_MODE {
                        println!("Finished executing opcodes, finishing...");
                    }
                    return Ok(InterpretOk);
                }
            };

            if DEBUG_MODE {
                Chunk::disassemble_instruction(&instruction, current_ip, &self.chunk.borrow());
                println!(" {}", *self.stack.borrow());
            }

            match instruction {
                OpCode::OpReturn(_) => {
                    let val = self.stack.borrow_mut().pop()?;
                    if DEBUG_MODE {
                        println!("Popped: {}", val);
                    } else {
                        println!("{}", val);
                    }
                }
                OpCode::OpConstant(constants_index) => {
                    let constant =
                        VM::read_constant(&self.chunk.borrow().constants.values, constants_index)
                            .expect(
                                format!(
                                    "Constant at IP {} did not return expected value!",
                                    current_ip
                                )
                                .as_str(),
                            );
                    self.stack.borrow_mut().push(constant);
                }
                OpCode::OpTrue => self.stack.borrow_mut().push(Value::Boolean(true)),
                OpCode::OpFalse => self.stack.borrow_mut().push(Value::Boolean(false)),
                OpCode::OpNil => self.stack.borrow_mut().push(Value::Nil),
                OpCode::OpNot => {
                    let val = self.stack.borrow_mut().pop()?;
                    self.stack.borrow_mut().push(self.is_falsey(val));
                }
                OpCode::OpNegate => {
                    let val = self.stack.borrow_mut().pop()?;

                    // check for non number types
                    let val = match val {
                        Value::Number(num) => Value::Number(num),
                        _ => {
                            return Err(InterpretError::RuntimeError(
                                "Cannot negate non-number type.".to_string(),
                            ))
                        }
                    };
                    self.stack.borrow_mut().push(-val);
                }
                OpCode::OpAdd => {
                    let b = self.stack.borrow_mut().pop()?; // rhs operand
                    let a = self.stack.borrow_mut().pop()?; // lhs operand

                    if self.check_for_strings(&a, &b) {
                        self.concatenate(a, b);
                    } else {
                        let (a, b) = self.check_for_non_number_types(a, b)?;
                        self.stack.borrow_mut().push(a + b); // push result
                    }
                }
                OpCode::OpSubtract => {
                    let b = self.stack.borrow_mut().pop()?; // rhs operand
                    let a = self.stack.borrow_mut().pop()?; // lhs operand
                    let (a, b) = self.check_for_non_number_types(a, b)?;
                    self.stack.borrow_mut().push(a - b); // push result
                }
                OpCode::OpMultiply => {
                    let b = self.stack.borrow_mut().pop()?; // rhs operand
                    let a = self.stack.borrow_mut().pop()?; // lhs operand
                    let (a, b) = self.check_for_non_number_types(a, b)?;
                    self.stack.borrow_mut().push(a * b); // push result
                }
                OpCode::OpDivide => {
                    let b = self.stack.borrow_mut().pop()?; // rhs operand
                    let a = self.stack.borrow_mut().pop()?; // lhs operand
                    let (a, b) = self.check_for_non_number_types(a, b)?;
                    self.stack.borrow_mut().push(a / b); // push result
                }
                OpCode::OpEqual => {
                    let b = self.stack.borrow_mut().pop()?; // rhs
                    let a = self.stack.borrow_mut().pop()?; // lhs
                    self.stack.borrow_mut().push(Value::Boolean(a == b));
                }
                OpCode::OpGreater => {
                    let b = self.stack.borrow_mut().pop()?; // rhs operand
                    let a = self.stack.borrow_mut().pop()?; // lhs operand
                    let (a, b) = self.check_for_non_number_types(a, b)?;
                    self.stack.borrow_mut().push(Value::Boolean(a > b)); // push result
                }
                OpCode::OpLess => {
                    let b = self.stack.borrow_mut().pop()?; // rhs operand
                    let a = self.stack.borrow_mut().pop()?; // lhs operand
                    let (a, b) = self.check_for_non_number_types(a, b)?;
                    self.stack.borrow_mut().push(Value::Boolean(a < b)); // push result
                }
            }
        }
    }

    fn is_falsey(&self, value: Value) -> Value {
        match value {
            Value::Boolean(false) | Value::Nil => Value::Boolean(true),
            _ => Value::Boolean(false),
        }
    }

    fn concatenate(&self, lhs: Value, rhs: Value) {
        match lhs {
            Value::Object(obj) => match obj.object_type {
                ObjectType::ObjString(string_one) => match rhs {
                    Value::Object(obj_two) => match obj_two.object_type {
                        ObjectType::ObjString(string_two) => {
                            let new_string_obj = Value::Object(RoxObject::new(
                                ObjectType::ObjString(string_one + string_two),
                            ));

                            self.stack.borrow_mut().push(new_string_obj);
                        }
                    },
                    _ => (),
                },
            },
            _ => (),
        }
    }

    fn check_for_strings(&self, lhs: &Value, rhs: &Value) -> bool {
        match lhs {
            Value::Object(obj_one) => match obj_one.object_type {
                ObjectType::ObjString(_) => match rhs {
                    Value::Object(obj_two) => match obj_two.object_type {
                        ObjectType::ObjString(_) => true,
                    },
                    _ => false,
                },
            },
            _ => false,
        }
    }

    fn check_for_non_number_types(
        &self,
        a: Value,
        b: Value,
    ) -> Result<(Value, Value), InterpretError> {
        let a = match a {
            Value::Number(num) => Value::Number(num),
            _ => {
                let line = self.chunk.borrow().get_line(*self.ip.borrow() - 1);
                return Err(InterpretError::RuntimeError(format!(
                    "[line {}]: Cannot relate two non-number types",
                    line
                )));
            }
        };
        let b = match b {
            Value::Number(num) => Value::Number(num),
            _ => {
                let line = self.chunk.borrow().get_line(*self.ip.borrow() - 1);
                return Err(InterpretError::RuntimeError(format!(
                    "[line {}]: Cannot relate two non-number types",
                    line
                )));
            }
        };

        Ok((a, b))
    }

    pub fn interpret(&'a self, source: &str) -> InterpretResult {
        // read and scan tokens
        let tokens = self.scanner.scan_tokens(source);

        // make new compiler
        let chunk = Rc::clone(&self.chunk);
        let peekable_tokens = RefCell::new(tokens.iter().peekable());
        let compiler = Compiler::new(
            chunk,
            peekable_tokens,
            Box::new(|new_object| self.add_object(new_object)),
        );

        // parse and compile tokens into opcodes
        if !compiler.compile() {
            return Err(InterpretError::CompileError(
                "Compiler error in VM interpreter.".to_string(),
            ));
        }

        if DEBUG_MODE {
            println!("Objects:");
            //self.print_objects();
            print!("|  IP  | Line | OpCode              | Stack\n");
        }
        // run vm with chunk filled with compiled opcodes
        self.run()
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use super::*;
    use crate::RoxNumber;

    #[test]
    fn test_vm() {
        let vm = VM::new();
        vm.interpret("1 + 2");
        todo!()
    }

    #[test]
    fn test_negate_op() {
        let vm = VM::new();
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(45.0)), 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpNegate, 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpReturn(0), 1);

        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values.len(), 1);
        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values.len(), 1);
        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(vm.stack.borrow().values.len(), 0);
    }

    #[test]
    fn test_add_binary_op() {
        let vm = VM::new();
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(45.0)), 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(15.0)), 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpAdd, 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpReturn(12), 1);

        vm.interpret(&"".to_string()).unwrap();
        vm.interpret(&"".to_string()).unwrap();
        vm.interpret(&"".to_string()).unwrap();
        assert_eq!(
            vm.stack.borrow().values[0],
            Some(Value::Number(RoxNumber(60.0)))
        );
    }

    #[test]
    fn test_mult_op_1() {
        let vm = VM::new();
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(1.0)), 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(2.0)), 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpMultiply, 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(3.0)), 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpAdd, 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.borrow().code.len() - 1 {
            vm.interpret(&"".to_string()).unwrap();
        }

        assert_eq!(
            vm.stack.borrow().values[0],
            Some(Value::Number(RoxNumber(5.0)))
        );
    }

    /// Test 1 + 2 * 3 == 7
    #[test]
    fn test_mult_op_2() {
        let vm = VM::new();
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(1.0)), 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(2.0)), 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(3.0)), 1);

        vm.chunk.borrow_mut().write_chunk(OpCode::OpMultiply, 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpAdd, 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.borrow().code.len() - 1 {
            vm.interpret(&"".to_string()).unwrap();
        }

        assert_eq!(
            vm.stack.borrow().values[0],
            Some(Value::Number(RoxNumber(7.0)))
        );
    }

    /// Test 3 - 2 - 1 == 0
    #[test]
    fn test_sub() {
        let vm = VM::new();
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(3.0)), 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(2.0)), 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpSubtract, 1);
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(1.0)), 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpSubtract, 1);
        vm.chunk.borrow_mut().write_chunk(OpCode::OpReturn(1), 1);

        for _ in 0..vm.chunk.borrow().code.len() - 1 {
            vm.interpret(&"".to_string()).unwrap();
        }

        assert_eq!(
            vm.stack.borrow().values[0],
            Some(Value::Number(RoxNumber(0.0)))
        );
    }

    #[test]
    fn test_order_operations() {
        let vm = VM::new();
        vm.chunk
            .borrow_mut()
            .add_constant(Value::Number(RoxNumber(1.0)), 1);
    }
}
