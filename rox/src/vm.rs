use crate::Chunk;
use crate::Compiler;
use crate::ObjectList;
use crate::ObjectType;
use crate::OpCode;
use crate::RcMut;
use crate::RoxMap;
use crate::RoxObject;
use crate::RoxString;
use crate::Scanner;
use crate::Stack;
use crate::Table;
use crate::Value;
use crate::DEBUG_MODE;
use crate::{InterpretError, InterpretOk, InterpretResult};
use std::cell::RefCell;
use std::rc::Rc;

pub struct VM {
    pub chunk: RcMut<Chunk>,
    ip: RefCell<usize>,
    stack: RefCell<Stack>,
    scanner: Scanner,
    objects: Rc<RefCell<ObjectList>>,
    globals: RcMut<Table<RoxString, Value>>,
}

impl VM {
    pub fn new() -> VM {
        let objects = Rc::new(RefCell::new(ObjectList::new()));
        let chunk = Rc::new(RefCell::new(Chunk::new(Rc::clone(&objects))));
        VM {
            chunk: Rc::clone(&chunk),
            ip: RefCell::new(0),
            stack: RefCell::new(Stack::new()),
            scanner: Scanner::new(),
            objects: Rc::clone(&objects),
            globals: Rc::new(RefCell::new(Table::new())),
        }
    }

    pub fn reset(&mut self) {
        *(self.ip.borrow_mut()) = 0;
        self.chunk.borrow_mut().reset();
        self.objects.borrow_mut().reset();

        let t = self.globals.borrow_mut();
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

    fn incr_ip(&self) -> usize {
        let current_ip = self.ip.borrow().clone();
        *self.ip.borrow_mut() += 1;

        current_ip
    }

    fn run(&self) -> InterpretResult {
        loop {
            let current_ip = self.incr_ip();

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

                    // check for string concatenation
                    if let (true, Some(str_1), Some(str_2)) = self.check_for_strings(&a, &b) {
                        self.concatenate(str_1, str_2);
                    } else {
                        // otherwise only numbers are addable
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

    fn concatenate<'a>(&self, lhs: &'a RoxString, rhs: &'a RoxString) {
        let new_string = lhs.clone() + rhs.clone();
        let mut new_string_obj = RoxObject::new(ObjectType::ObjString(new_string));
        // new string is allocated so add it to objects list
        self.objects.borrow_mut().add_object(&mut new_string_obj);
        self.stack.borrow_mut().push(Value::Object(new_string_obj));
    }

    fn check_for_strings<'a>(
        &self,
        lhs: &'a Value,
        rhs: &'a Value,
    ) -> (bool, Option<&'a RoxString>, Option<&'a RoxString>) {
        match lhs {
            Value::Object(obj_one) => match &obj_one.object_type {
                ObjectType::ObjString(str_1) => match rhs {
                    Value::Object(obj_two) => match &obj_two.object_type {
                        ObjectType::ObjString(str_2) => (true, Some(&str_1), Some(&str_2)),
                    },
                    _ => (false, None, None),
                },
            },
            _ => (false, None, None),
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

    pub fn interpret(&self, source: &str) -> InterpretResult {
        // read and scan tokens
        let tokens = self.scanner.scan_tokens(source);

        // make new compiler
        let chunk = Rc::clone(&self.chunk);
        let peekable_tokens = RefCell::new(tokens.iter().peekable());
        let compiler = Compiler::new(chunk, peekable_tokens);

        // parse and compile tokens into opcodes
        if !compiler.compile() {
            return Err(InterpretError::CompileError(
                "Compiler error in VM interpreter.".to_string(),
            ));
        }

        if DEBUG_MODE {
            println!("Objects:");
            self.objects.borrow().print_objects();
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
