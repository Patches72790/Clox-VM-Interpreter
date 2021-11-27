use rox::{Chunk, OpCode, Value};

fn main() {
    let mut my_c = Chunk::new();
    // adding some return opcodes
    my_c.write_chunk(OpCode::OpReturn(8), 1);
    my_c.write_chunk(OpCode::OpReturn(22), 1);
    my_c.write_chunk(OpCode::OpReturn(55), 1);

    // adding constant value
    let index = my_c.add_constant(Value::Number(45.0));
    my_c.write_chunk(OpCode::OpConstant(index), 1);

    my_c.write_chunk(OpCode::OpReturn(12), 4);

    // TODO -- Think of better way of adding in blank lines for lines array?
    let index = my_c.add_constant(Value::Number(69.0));
    my_c.write_chunk(OpCode::OpConstant(index), 6);
    // debug print instructions
    my_c.disassemble_chunk("my chunk");
}
