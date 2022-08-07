use crate::opcode::VariableOp;
use crate::{ObjectList, ObjectType, OpCode, RcMut, RoxObject, RoxString, Table, DEBUG_MODE};
use crate::{Value, Values};
use std::cell::RefCell;
use std::rc::Rc;
use string_interner::StringInterner;

thread_local! {
    static INTERNER: StringInterner = StringInterner::default();
}

///The Chunk type corresponds to the basic block
///of code with a size of count and capacity.
///The code vector corresponds to the list of instructions
///of type OpCode.
#[derive(Debug)]
pub struct Chunk {
    count: usize,
    pub code: Vec<OpCode>,
    pub constants: Values,
    pub lines: Vec<String>,
    objects: Rc<RefCell<ObjectList>>,
    global_indices: RcMut<Table<RoxString, usize>>,
}

impl Chunk {
    ///
    ///Creates and returns a new chunk with size/capacity of 0.
    ///The code vector is initially set to Option<None>.
    ///
    pub fn new(
        objects: Rc<RefCell<ObjectList>>,
        global_indices: RcMut<Table<RoxString, usize>>,
    ) -> Chunk {
        Chunk {
            count: 0,
            code: vec![],
            constants: Values::new(),
            lines: vec![],
            objects,
            global_indices,
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.code = vec![];
        //self.constants = Values::new();
        self.lines = vec![];
        //self.global_indices.borrow_mut().reset();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    ///
    ///Writes the specified byte to the instruction code vector
    ///contained within this Chunk and increments the count.
    ///If the size exceeds the capacity, the capacity is
    ///automatically increased to account for the increase.
    ///
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        Chunk::write_line_info(self, line, &byte);
        self.count += 1;
    }

    ///
    ///Writes the line info for each byte code instruction to the chunk's
    ///line vector for keeping track of line data.
    ///
    fn write_line_info(chunk: &mut Chunk, line: usize, byte: &OpCode) {
        if line > chunk.lines.len() {
            // possibly add blank lines?
            let num_blank_lines = line - (chunk.lines.len() + 1);

            // this is a new line
            let new_count = match chunk.lines.last() {
                // add up the previous lines count
                Some(val) => val.parse::<i32>().unwrap() + 1,
                None => 1,
            };

            // TODO -- how not to pad lines with count for empty lines?
            for _ in 0..num_blank_lines {
                chunk.lines.push((new_count - 1).to_string());
            }

            chunk.lines.push(new_count.to_string());
        } else {
            // increment number of instructions in current line
            let current_line = match chunk.lines.pop() {
                Some(val) => val,
                None => panic!(
                    "No string index value for the current line {} and byte {}!",
                    line, byte
                ),
            };
            let mut num_bytes_in_line = current_line.parse::<i32>().unwrap();
            num_bytes_in_line += 1;
            chunk.lines.push(num_bytes_in_line.to_string());
        }
    }

    ///
    /// Searches in the lines array for the first element
    /// that is greater than the index of the chunk byte
    /// being searched for and panics if not found.
    ///
    pub fn get_line(&self, index: usize) -> usize {
        let result = self
            .lines
            .iter()
            .enumerate()
            .find(|(_, line)| index < line.parse::<usize>().unwrap());
        if let Some(val) = result {
            return val.0 + 1;
        }
        panic!("Index {} not in lines list!", index);
    }

    ///
    ///This debug function disassembles the Chunk by iterating through
    ///its list of bytecode instructions.
    ///
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        for (offset, byte) in self.code.iter().enumerate() {
            // offset into code vector is just the index
            Chunk::disassemble_instruction(byte, offset, self);
        }
    }

    ///
    /// Convenience method for writing value to the constants Values array inside Chunk.
    /// Then the method writes to the chunk with the provided index.
    ///
    pub fn add_constant(&mut self, value: Value, line: usize) {
        let (index, value_ref) = self.constants.write_value(value, None);

        // add rox object to list for tracking allocated objects
        if let Value::Object(obj) = value_ref {
            self.objects.borrow_mut().add_object(obj);
        }

        self.write_chunk(OpCode::OpConstant(index), line);
    }

    pub fn add_identifier_constant(
        &mut self,
        string_value: &RoxString,
        line: usize,
        variable_op: VariableOp,
    ) -> usize {
        let (index, value_ref) = self.constants.write_value(
            Value::Object(RoxObject::new(ObjectType::ObjString(string_value.clone()))),
            Some(&mut self.global_indices.borrow_mut()),
        );
        if DEBUG_MODE {
            println!("Added id {} at index {} to values", value_ref, index);
        }

        if let Value::Object(obj) = value_ref {
            self.objects.borrow_mut().add_object(obj);
        }

        match variable_op {
            VariableOp::GetGlobal => self.write_chunk(OpCode::OpGetGlobal(index), line),
            VariableOp::SetGlobal => self.write_chunk(OpCode::OpSetGlobal(index), line),
            VariableOp::Define => (), // define globals opcode defers
                                      //writing opcode until after parsing expression
        }

        index
    }

    ///Helper function for disassembling bytecode instructions instructions
    ///in the bytecode vector for Chunk.
    pub fn disassemble_instruction(instr: &OpCode, offset: usize, chunk: &Chunk) {
        print!("| {:0>4} ", offset);
        print!("| {:>4} | ", chunk.get_line(offset));

        match instr {
            OpCode::OpReturn(_) => Chunk::simple_instruction("OP_RETURN"),
            OpCode::OpConstant(constants_index) => {
                Chunk::constant_instruction("OP_CONSTANT", *constants_index, chunk)
            }
            OpCode::OpNegate => Chunk::simple_instruction("OP_NEGATE"),
            OpCode::OpAdd => Chunk::simple_instruction("OP_ADD"),
            OpCode::OpSubtract => Chunk::simple_instruction("OP_SUBTRACT"),
            OpCode::OpMultiply => Chunk::simple_instruction("OP_MULTIPLY"),
            OpCode::OpDivide => Chunk::simple_instruction("OP_DIVIDE"),
            OpCode::OpNil => Chunk::simple_instruction("OP_NIL"),
            OpCode::OpTrue => Chunk::simple_instruction("OP_TRUE"),
            OpCode::OpFalse => Chunk::simple_instruction("OP_FALSE"),
            OpCode::OpNot => Chunk::simple_instruction("OP_NOT"),
            OpCode::OpGreater => Chunk::simple_instruction("OP_GREATER"),
            OpCode::OpEqual => Chunk::simple_instruction("OP_EQUAL"),
            OpCode::OpLess => Chunk::simple_instruction("OP_LESS"),
            OpCode::OpPrint => Chunk::simple_instruction("OP_PRINT"),
            OpCode::OpPop => Chunk::simple_instruction("OP_POP"),
            OpCode::OpDefineGlobal(_) => Chunk::simple_instruction("OP_DEFINE_GLOBAL"),
            OpCode::OpGetGlobal(_) => Chunk::simple_instruction("OP_GET_GLOBAL"),
            OpCode::OpSetGlobal(_) => Chunk::simple_instruction("OP_SET_GLOBAL"),
            OpCode::OpGetLocal(_) => Chunk::simple_instruction("OP_GET_LOCAL"),
            OpCode::OpSetLocal(_) => Chunk::simple_instruction("OP_SET_LOCAL"),
            OpCode::OpJumpIfFalse(offset) => {
                Chunk::simple_instruction(format!("OP_JUMP_IF_FALSE {}", offset.unwrap()).as_str())
            }
        };
    }

    fn constant_instruction(name: &str, index: usize, chunk: &Chunk) {
        print!("{:>11} {:<4}'", name, index);
        match chunk.constants.values.get(index) {
            Some(val) => print!("{:>4}", val),
            None => panic!("No constant value at that index!"),
        };
        print!("' |");
    }

    fn simple_instruction(name: &str) {
        print!("{:<25} |", name);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_chunk() {}

    #[test]
    fn test_write_constants() {}

    #[test]
    fn test_disassemble_chunk() {}
}
