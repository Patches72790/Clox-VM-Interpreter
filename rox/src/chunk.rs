/**
 * The OpCodes for the Chunk instructions
 * to be used for the Rox VM
 */
#[derive(Debug)]
pub enum OpCode {
    OpReturn(usize),
}

#[derive(Debug)]
pub struct Instruction {
    op_code: OpCode,
    offset: usize,
}

/**
 * The Chunk type corresponds to the basic block
 * of code with a size of count and capacity.
 * The code vector corresponds to the list of instructions
 * of type OpCode.
 */
#[derive(Debug)]
pub struct Chunk {
    count: i32,
    code: Vec<Instruction>,
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
        }
    }

    /**
     * Writes the specified byte to the instruction code vector
     * contained within this Chunk and increments the count.
     * If the size exceeds the capacity, the capacity is
     * automatically increased to account for the increase.
     */
    pub fn write_chunk(&mut self, byte: OpCode) {
        let new_instr = Instruction {
            op_code: byte,
            offset: self.count as usize,
        };
        self.code.push(new_instr);
        self.count += 1;
    }

    /**
     * This debug function disassembles the Chunk by iterating through
     * its list of bytecode instructions.
     */
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        for byte in self.code.iter() {
            offset = Chunk::disassemble_instruction(byte, offset);
        }
    }

    /**
     * Helper function for disassembling bytecode instructions instructions
     * in the bytecode vector for Chunk.
     */
    fn disassemble_instruction(instr: &Instruction, offset: usize) -> usize {
        print!("{:0>4} ", offset);

        match instr.op_code {
            OpCode::OpReturn(_) => Chunk::simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {:?}", instr);
                offset + 1
            }
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}
