use crate::OpCode;
use crate::{Value, Values};

/**
 * The Chunk type corresponds to the basic block
 * of code with a size of count and capacity.
 * The code vector corresponds to the list of instructions
 * of type OpCode.
 */
#[derive(Debug)]
pub struct Chunk {
    count: i32,
    code: Vec<OpCode>,
    constants: Values,
    lines: Vec<String>,
}

impl Chunk {
    /**
     * Creates and returns a new chunk with size/capacity of 0.
     * The code vector is initially set to Option<None>.
     */
    pub fn new() -> Chunk {
        Chunk {
            count: 0,
            code: vec![],
            constants: Values::new(),
            lines: vec![],
        }
    }

    /**
     * Writes the specified byte to the instruction code vector
     * contained within this Chunk and increments the count.
     * If the size exceeds the capacity, the capacity is
     * automatically increased to account for the increase.
     */
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte.clone());
        Chunk::write_line_info(self, line, &byte);
        self.count += 1;
    }

    /**
     * Writes the line info for each byte code instruction to the chunk's
     * line vector for keeping track of line data.
     */
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

    fn get_line(&self, index: usize) -> usize {
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

    /**
     * This debug function disassembles the Chunk by iterating through
     * its list of bytecode instructions.
     */
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        for (i, byte) in self.code.iter().enumerate() {
            offset = Chunk::disassemble_instruction(byte, offset, self);
        }
    }

    /**
     * Convenience method for writing value to the constants Values array inside Chunk.
     *
     * returns the index at which the value was added to the constants array
     */
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value(value)
    }

    /**
     * Helper function for disassembling bytecode instructions instructions
     * in the bytecode vector for Chunk.
     */
    fn disassemble_instruction(instr: &OpCode, offset: usize, chunk: &Chunk) -> usize {
        print!("{:0>4} ", offset);

        print!("{:>4} ", chunk.get_line(offset));
        print!("Lines: {:?} ", chunk.lines);

        match instr {
            OpCode::OpReturn(_) => Chunk::simple_instruction("OP_RETURN", offset),
            OpCode::OpConstant(constants_index) => {
                Chunk::constant_instruction("OP_CONSTANT", *constants_index, offset, chunk)
            }
        }
    }

    fn constant_instruction(name: &str, index: usize, offset: usize, chunk: &Chunk) -> usize {
        print!("{:>11} {:<4}'", name, index);
        match chunk.constants.values.get(index) {
            Some(val) => print!("{}", val),
            None => panic!("No constant value at that index!"),
        };
        print!("'\n");
        offset + 1
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write_chunk() {}

    #[test]
    fn test_write_op_return() {}

    #[test]
    fn test_disassemble_chunk() {
        let c = Chunk::new();
    }
}
