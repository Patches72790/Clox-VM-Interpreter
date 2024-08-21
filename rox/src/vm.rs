use crate::ObjectType;
use crate::OpCode;
use crate::RoxMap;
use crate::RoxObject;
use crate::RoxString;
use crate::Stack;
use crate::Table;
use crate::Value;
use crate::DEBUG_MODE;
use crate::{Chunk, Compiler};
use crate::{InterpretError, InterpretOk, InterpretResult};

#[derive(Debug)]
pub struct VM {
    ip: usize,
    stack: Stack<Value>,
    globals: Table<RoxString, Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            ip: 0,
            stack: Stack::new(),
            globals: Table::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.stack.reset();
    }

    fn read_byte(code: &[OpCode], ip: usize) -> Option<OpCode> {
        code.get(ip).copied()
    }

    fn read_constant(values: &[Value], index: usize) -> Option<Value> {
        values.get(index).cloned()
    }

    fn read_string(values: &[Value], str_id_index: usize) -> RoxString {
        let string_id = VM::read_constant(values, str_id_index).unwrap_or_else(|| {
            panic!("String id constant at index {str_id_index} did not return expected value!")
        });

        match string_id {
            Value::Object(obj) => match obj.object_type {
                ObjectType::ObjString(string) => string,
                _ => panic!(
                    "Error Object String type was not located at index {}",
                    str_id_index
                ),
            },
            _ => panic!(
                "Error Value object was not located at index {}",
                str_id_index
            ),
        }
    }

    fn incr_ip(&mut self) -> usize {
        let current_ip = self.ip;
        self.ip += 1;

        current_ip
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            let current_ip = self.incr_ip();

            // read next instruction
            let instruction = match VM::read_byte(&chunk.code, current_ip) {
                Some(instr) => instr,
                None => {
                    if DEBUG_MODE {
                        println!("Finished executing opcodes, finishing...");
                    }
                    return Ok(InterpretOk);
                }
            };

            if DEBUG_MODE {
                println!("Executing instruction: ");
                Chunk::disassemble_instruction(&instruction, current_ip, chunk);
                println!("\n\t{}", self.stack);
            }

            match instruction {
                OpCode::OpReturn(_) => {
                    // Nothing for now
                    //let val = self.stack.borrow_mut().pop()?;
                    //if DEBUG_MODE {
                    //    println!("Popped: {}", val);
                    //} else {
                    //    println!("{}", val);
                    //}
                }
                OpCode::OpPop => {
                    self.stack.pop();
                }
                OpCode::OpConstant(constants_index) => {
                    let constant = VM::read_constant(&chunk.constants.values, constants_index)
                        .unwrap_or_else(|| {
                            panic!(
                                "Constant at IP {} did not return expected value!",
                                current_ip
                            )
                        });
                    self.stack.push(constant);
                }
                OpCode::OpDefineGlobal(str_id_index) => {
                    let string_id = VM::read_string(&chunk.constants.values, str_id_index);

                    if DEBUG_MODE {
                        println!("Added id {string_id} to globals table");
                    }

                    let global_rhs = self.stack.peek().unwrap_or_else(|| {
                        panic!(
                            "Error getting stack value in DefineGlobal({})",
                            str_id_index
                        )
                    });
                    self.globals.set(&string_id, global_rhs);
                    self.stack.pop();
                }
                OpCode::OpSetGlobal(str_id_index) => {
                    let string_id = VM::read_string(&chunk.constants.values, str_id_index);

                    let rhs = self.stack.peek().expect("Error peeking stack in SetGlobal");
                    self.globals.set(&string_id, rhs);
                    if DEBUG_MODE {
                        println!("Set global id {string_id} to {rhs}.");
                    }
                }
                OpCode::OpGetGlobal(str_id_index) => {
                    let string_id = VM::read_string(&chunk.constants.values, str_id_index);

                    if let Some(value) = self.globals.get(&string_id) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(InterpretError::RuntimeError(format!(
                            "Undefined variable '{}'.",
                            string_id
                        )));
                    }

                    if DEBUG_MODE {
                        println!("Read global id {string_id} from globals table");
                    }
                }
                OpCode::OpGetLocal(index) => {
                    self.stack
                        .get_and_push_local(index)
                        .expect("Error getting local at index");
                }
                OpCode::OpSetLocal(index) => {
                    self.stack
                        .set_local(index)
                        .expect("Error setting local at index");
                }
                OpCode::OpTrue => self.stack.push(Value::Boolean(true)),
                OpCode::OpFalse => self.stack.push(Value::Boolean(false)),
                OpCode::OpNil => self.stack.push(Value::Nil),
                OpCode::OpNot => {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(self.is_falsey(val)));
                }
                OpCode::OpNegate => {
                    let val = self.stack.pop().unwrap();

                    // check for non number types
                    let val = match val {
                        Value::Number(num) => Value::Number(num),
                        _ => {
                            return Err(InterpretError::RuntimeError(
                                "Cannot negate non-number type.".to_string(),
                            ))
                        }
                    };
                    self.stack.push(-val);
                }
                OpCode::OpAdd => {
                    let b = self.stack.pop().unwrap(); // rhs operand
                    let a = self.stack.pop().unwrap(); // lhs operand

                    // check for string concatenation
                    if let (true, Some(str_1), Some(str_2)) = self.check_for_strings(&a, &b) {
                        self.concatenate(str_1, str_2);
                    } else {
                        // otherwise only numbers are addable
                        let (a, b) = self.check_for_non_number_types(chunk, a, b)?;
                        self.stack.push(a + b); // push result
                    }
                }
                OpCode::OpSubtract => {
                    let b = self.stack.pop().unwrap(); // rhs operand
                    let a = self.stack.pop().unwrap(); // lhs operand
                    let (a, b) = self.check_for_non_number_types(chunk, a, b)?;
                    self.stack.push(a - b); // push result
                }
                OpCode::OpMultiply => {
                    let b = self.stack.pop().unwrap(); // rhs operand
                    let a = self.stack.pop().unwrap(); // lhs operand
                    let (a, b) = self.check_for_non_number_types(chunk, a, b)?;
                    self.stack.push(a * b); // push result
                }
                OpCode::OpDivide => {
                    let b = self.stack.pop().unwrap(); // rhs operand
                    let a = self.stack.pop().unwrap(); // lhs operand
                    let (a, b) = self.check_for_non_number_types(chunk, a, b)?;
                    self.stack.push(a / b); // push result
                }
                OpCode::OpEqual => {
                    let b = self.stack.pop().unwrap(); // rhs
                    let a = self.stack.pop().unwrap(); // lhs
                    self.stack.push(Value::Boolean(a == b));
                }
                OpCode::OpGreater => {
                    let b = self.stack.pop().unwrap(); // rhs operand
                    let a = self.stack.pop().unwrap(); // lhs operand
                    let (a, b) = self.check_for_non_number_types(chunk, a, b)?;
                    self.stack.push(Value::Boolean(a > b)); // push result
                }
                OpCode::OpLess => {
                    let b = self.stack.pop().unwrap(); // rhs operand
                    let a = self.stack.pop().unwrap(); // lhs operand
                    let (a, b) = self.check_for_non_number_types(chunk, a, b)?;
                    self.stack.push(Value::Boolean(a < b)); // push result
                }
                OpCode::OpPrint => {
                    println!("{}", self.stack.pop().unwrap());
                }
                OpCode::OpJumpIfFalse(jump) => {
                    let jump_offset =
                        jump.unwrap_or_else(|| panic!("Unknown jump offset for JumpIfFalse"));
                    if self.is_falsey(
                        self.stack
                            .peek()
                            .unwrap_or_else(|| {
                                panic!("Error peeking stack in JumpIfFalse({:?})", jump)
                            })
                            .clone(),
                    ) {
                        self.ip += jump_offset;
                    }
                }
                OpCode::OpJump(jump) => {
                    let jump_offset = jump.unwrap();
                    self.ip += jump_offset;
                }
                OpCode::OpLoop(jump) => {
                    self.ip -= jump;
                }
            }
        }
    }

    fn is_falsey(&self, value: Value) -> bool {
        matches!(value, Value::Boolean(false) | Value::Nil)
    }

    fn concatenate<'a>(&mut self, lhs: &'a RoxString, rhs: &'a RoxString) {
        let new_string = lhs.clone() + rhs.clone();
        let new_string_obj = RoxObject::new(ObjectType::ObjString(new_string));
        self.stack.push(Value::Object(new_string_obj));
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
                        ObjectType::ObjString(str_2) => (true, Some(str_1), Some(str_2)),
                    },
                    _ => (false, None, None),
                },
            },
            _ => (false, None, None),
        }
    }

    fn check_for_non_number_types(
        &self,
        chunk: &Chunk,
        a: Value,
        b: Value,
    ) -> Result<(Value, Value), InterpretError> {
        let a = match a {
            Value::Number(num) => Value::Number(num),
            _ => {
                let line = chunk.get_line(self.ip - 1);
                return Err(InterpretError::RuntimeError(format!(
                    "[line {}]: Cannot relate two non-number types: a=({}) b=({})",
                    line, a, b
                )));
            }
        };
        let b = match b {
            Value::Number(num) => Value::Number(num),
            _ => {
                let line = chunk.get_line(self.ip - 1);
                return Err(InterpretError::RuntimeError(format!(
                    "[line {}]: Cannot relate two non-number types {} {}",
                    line, a, b
                )));
            }
        };

        Ok((a, b))
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let chunk = match Compiler::compile(source) {
            Ok(chunk) => chunk,
            Err(msg) => {
                return Err(InterpretError::CompileError(format!(
                    "Compiler error in VM interpreter: {}",
                    msg
                )))
            }
        };

        if DEBUG_MODE {
            chunk.disassemble_chunk("OpCode Debug");
        }

        // run vm with chunk filled with compiled opcodes
        self.run(&chunk)
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error;

    #[test]
    fn test_negate_op() {
        let mut vm = VM::new();
        let result = vm.interpret("-45;").unwrap();

        assert!(result == error::InterpretOk);
    }

    #[test]
    fn test_add_binary_op() {
        let mut vm = VM::new();

        if let Err(msg) = vm.interpret("45 + 15;") {
            panic!("{}", msg)
        };
    }

    /// Test 1 + 2 * 3 == 7
    #[test]
    fn test_mult_op_1() {
        let mut vm = VM::new();

        if let Err(msg) = vm.interpret("1 + 2 * 3;") {
            panic!("{}", msg)
        }
    }

    /// Test 3 - 2 - 1 == 0
    #[test]
    fn test_sub() {
        let mut vm = VM::new();

        if let Err(msg) = vm.interpret("3 - 2 - 1;") {
            panic!("{}", msg)
        }
    }

    #[test]
    fn test_local_vars() {
        let mut vm = VM::new();
        if let Err(msg) = vm.interpret("var a = 123; { var b = 456; print a + b; } print a;") {
            panic!("{}", msg)
        }
    }

    #[test]
    fn test_while_loop_simple() {
        let mut vm = VM::new();
        if let Err(msg) = vm.interpret("var a = 123; while (a > 125) {  a = a + 1; } print a;") {
            panic!("{}", msg)
        }
    }

    #[test]
    fn test_for_loop_simple() {
        let mut vm = VM::new();
        if let Err(msg) = vm.interpret("for (var a = 0; a > 125; a = a+1) { print a;}") {
            panic!("{}", msg)
        }
    }

    #[test]
    fn test_for_loop_global() {
        let mut vm = VM::new();
        if let Err(msg) = vm.interpret("var a = 3; for (; a > 0;) { print a; a = a - 1;}") {
            panic!("{}", msg)
        }
    }

    #[test]
    fn test_for_loop_decr() {
        let mut vm = VM::new();
        // TODO =>  Error related to decremenet when for-loop var set inside for declaration
        // statement
        if let Err(msg) = vm.interpret("for (var a = 3; a > 2; a = a - 1) { print a; }") {
            panic!("{}", msg)
        }
    }
}
