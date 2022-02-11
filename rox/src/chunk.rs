use crate::OpCode;
use crate::DEBUG_MODE;
use crate::{Value, Values};

///The Chunk type corresponds to the basic block
///of code with a size of count and capacity.
///The code vector corresponds to the list of instructions
///of type OpCode.
#[derive(Debug)]
pub struct Chunk {
    count: i32,
    pub code: Vec<OpCode>,
    pub constants: Values,
    pub lines: Vec<String>,
}

impl Chunk {
    ///
    ///Creates and returns a new chunk with size/capacity of 0.
    ///The code vector is initially set to Option<None>.
    ///
    pub fn new() -> Chunk {
        Chunk {
            count: 0,
            code: vec![],
            constants: Values::new(),
            lines: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.code = vec![];
        self.constants = Values::new();
        self.lines = vec![];
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
                chunk.lines.push(String::from((new_count - 1).to_string()));
            }

            chunk.lines.push(String::from(new_count.to_string()));
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
        let index = self.constants.write_value(value);
        self.write_chunk(OpCode::OpConstant(index), line);
    }

    ///Helper function for disassembling bytecode instructions instructions
    ///in the bytecode vector for Chunk.
    pub fn disassemble_instruction(instr: &OpCode, offset: usize, chunk: &Chunk) {
        print!("| {:0>4} ", offset);
        print!("| {:>4} | ", chunk.get_line(offset));

        //        if DEBUG_MODE {
        //            print!("| Lines: {:?} |", chunk.lines);
        //        }
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
        print!("{:<19} |", name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RoxNumber;
    #[test]
    fn test_write_chunk() {
        let mut my_c = Chunk::new();

        my_c.write_chunk(OpCode::OpReturn(8), 1);
        my_c.write_chunk(OpCode::OpReturn(22), 1);
        my_c.write_chunk(OpCode::OpReturn(55), 1);
        my_c.add_constant(Value::Number(RoxNumber(69.0)), 2);
        my_c.add_constant(Value::Number(RoxNumber(42.0)), 2);

        assert_eq!(my_c.count, 5);
        assert_eq!(my_c.lines.len(), 2);
    }

    #[test]
    fn test_write_constants() {
        let mut my_c = Chunk::new();

        my_c.add_constant(Value::Number(RoxNumber(69.0)), 1);
        my_c.add_constant(Value::Number(RoxNumber(42.0)), 1);
        my_c.add_constant(Value::Number(RoxNumber(35.0)), 1);

        assert_eq!(my_c.constants.values.len(), 3);
    }

    #[test]
    fn test_disassemble_chunk() {
        let mut my_c = Chunk::new();

        my_c.write_chunk(OpCode::OpReturn(8), 1);
        my_c.write_chunk(OpCode::OpReturn(22), 1);
        my_c.write_chunk(OpCode::OpReturn(55), 1);
        my_c.add_constant(Value::Number(RoxNumber(69.0)), 2);
        my_c.add_constant(Value::Number(RoxNumber(42.0)), 2);
    }
}
