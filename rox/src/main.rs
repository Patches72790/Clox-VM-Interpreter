use rox::{Chunk, OpCode, Value};

fn main() {
    let mut my_c = Chunk::new();
    // adding some return opcodes
    my_c.write_chunk(OpCode::OpReturn(8));
    my_c.write_chunk(OpCode::OpReturn(22));

    // adding constant value
    let index = my_c.add_constant(Value::Number(45.0));
    my_c.write_chunk(OpCode::OpConstant(index));

    // debug print instructions
    my_c.disassemble_chunk("my chunk");
}
