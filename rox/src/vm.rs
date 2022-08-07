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
    global_indices: RcMut<Table<RoxString, usize>>,
}

impl VM {
    pub fn new() -> VM {
        let objects = Rc::new(RefCell::new(ObjectList::new()));
        let global_indices = Rc::new(RefCell::new(Table::new()));
        let chunk = Rc::new(RefCell::new(Chunk::new(
            Rc::clone(&objects),
            Rc::clone(&global_indices),
        )));
        VM {
            chunk: Rc::clone(&chunk),
            ip: RefCell::new(0),
            stack: RefCell::new(Stack::new()),
            scanner: Scanner::new(),
            objects: Rc::clone(&objects),
            globals: Rc::new(RefCell::new(Table::new())),
            global_indices: Rc::clone(&global_indices),
        }
    }

    pub fn reset(&mut self) {
        *(self.ip.borrow_mut()) = 0;
        self.chunk.borrow_mut().reset();
        self.objects.borrow_mut().reset();
        self.stack.borrow_mut().reset_stack();
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

    fn incr_ip(&self) -> usize {
        let current_ip = *self.ip.borrow();
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
                    // Nothing for now
                    //let val = self.stack.borrow_mut().pop()?;
                    //if DEBUG_MODE {
                    //    println!("Popped: {}", val);
                    //} else {
                    //    println!("{}", val);
                    //}
                }
                OpCode::OpPop => {
                    self.stack.borrow_mut().pop()?;
                }
                OpCode::OpConstant(constants_index) => {
                    let constant =
                        VM::read_constant(&self.chunk.borrow().constants.values, constants_index)
                            .unwrap_or_else(|| {
                                panic!(
                                    "Constant at IP {} did not return expected value!",
                                    current_ip
                                )
                            });
                    self.stack.borrow_mut().push(constant);
                }
                OpCode::OpDefineGlobal(str_id_index) => {
                    let string_id =
                        VM::read_string(&self.chunk.borrow().constants.values, str_id_index);

                    if DEBUG_MODE {
                        println!("Added id {string_id} to globals table");
                    }

                    let global_rhs = self.stack.borrow().peek(0)?;
                    self.globals.borrow_mut().set(&string_id, &global_rhs);
                    self.stack.borrow_mut().pop()?;
                }
                OpCode::OpSetGlobal(str_id_index) => {
                    let string_id =
                        VM::read_string(&self.chunk.borrow().constants.values, str_id_index);

                    let rhs = self.stack.borrow().peek(0)?;
                    if !self.globals.borrow_mut().get_and_set(&string_id, &rhs) {
                        return Err(InterpretError::RuntimeError(format!(
                            "Undefined variable {}",
                            string_id
                        )));
                    }
                    if DEBUG_MODE {
                        println!("Set global id {string_id} to {rhs}.");
                    }
                }
                OpCode::OpGetGlobal(str_id_index) => {
                    let string_id =
                        VM::read_string(&self.chunk.borrow().constants.values, str_id_index);

                    if let Some(value) = self.globals.borrow_mut().get(&string_id) {
                        self.stack.borrow_mut().push(value.clone());
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
                    if let Err(msg) = self.stack.borrow_mut().get_and_push_local(index) {
                        return Err(InterpretError::RuntimeError(msg.to_string()));
                    }
                }
                OpCode::OpSetLocal(index) => {
                    if let Err(msg) = self.stack.borrow_mut().set_local(index) {
                        return Err(InterpretError::RuntimeError(msg.to_string()));
                    }
                }
                OpCode::OpTrue => self.stack.borrow_mut().push(Value::Boolean(true)),
                OpCode::OpFalse => self.stack.borrow_mut().push(Value::Boolean(false)),
                OpCode::OpNil => self.stack.borrow_mut().push(Value::Nil),
                OpCode::OpNot => {
                    let val = self.stack.borrow_mut().pop()?;
                    self.stack
                        .borrow_mut()
                        .push(Value::Boolean(self.is_falsey(val)));
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
                OpCode::OpPrint => {
                    println!("{}", self.stack.borrow_mut().pop()?);
                }
                OpCode::OpJumpIfFalse(jump) => {
                    let jump_offset = jump.unwrap();
                    if self.is_falsey(self.stack.borrow().peek(0)?) {
                        *self.ip.borrow_mut() += jump_offset;
                    }
                }
                OpCode::OpJump(jump) => {
                    let jump_offset = jump.unwrap();
                    *self.ip.borrow_mut() += jump_offset;
                }
            }
        }
    }

    fn is_falsey(&self, value: Value) -> bool {
        matches!(value, Value::Boolean(false) | Value::Nil)
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
            println!("|  IP  | Line | OpCode                    | Stack");
        }
        // run vm with chunk filled with compiled opcodes
        self.run()
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
        let vm = VM::new();
        let result = vm.interpret("-45;").unwrap();

        assert!(result == error::InterpretOk);
    }

    #[test]
    fn test_add_binary_op() {
        let vm = VM::new();

        if let Err(msg) = vm.interpret("45 + 15;") {
            panic!("{}", msg)
        };
    }

    /// Test 1 + 2 * 3 == 7
    #[test]
    fn test_mult_op_1() {
        let vm = VM::new();

        if let Err(msg) = vm.interpret("1 + 2 * 3;") {
            panic!("{}", msg)
        }
    }

    /// Test 3 - 2 - 1 == 0
    #[test]
    fn test_sub() {
        let vm = VM::new();

        if let Err(msg) = vm.interpret("3 - 2 - 1;") {
            panic!("{}", msg)
        }
    }

    #[test]
    fn test_local_vars() {
        let vm = VM::new();
        if let Err(msg) = vm.interpret("var a = 123; { var b = 456; print a + b; } print a;") {
            panic!("{}", msg)
        }
    }
}
