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
    lines: String,
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
            lines: String::from(""),
        }
    }

    /**
     * Writes the specified byte to the instruction code vector
     * contained within this Chunk and increments the count.
     * If the size exceeds the capacity, the capacity is
     * automatically increased to account for the increase.
     */
    pub fn write_chunk(&mut self, byte: OpCode) {
        self.code.push(byte);

        let last_index = self.lines.split("L").last();
        let last_index = match last_index {
            Some(val) => val,
            None => panic!("error no value in last"),
        };

        let _ = last_index.replace(last_index, &self.count.to_string());

        self.lines.push_str(&(self.count.to_string() + "_"));
        self.count += 1;
    }

    fn get_line(&self, index: usize) -> usize {
        unimplemented!()
    }

    /**
     * This debug function disassembles the Chunk by iterating through
     * its list of bytecode instructions.
     */
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        for byte in self.code.iter() {
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

        match instr {
            OpCode::OpReturn(_) => Chunk::simple_instruction("OP_RETURN", offset),
            OpCode::OpConstant(constants_index) => {
                Chunk::constant_instruction("OP_CONSTANT", *constants_index, offset, chunk)
            }
            _ => {
                println!("Unknown opcode {:?}", instr);
                offset + 1
            }
        }
    }

    fn constant_instruction(name: &str, index: usize, offset: usize, chunk: &Chunk) -> usize {
        print!("{:>16} {:<4}'", name, index);
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
