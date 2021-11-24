use rox::{Chunk, OpCode};

fn main() {
    let mut my_c = Chunk::new();
    my_c.write_chunk(OpCode::OpReturn(8));
    my_c.write_chunk(OpCode::OpReturn(22));
    my_c.write_chunk(OpCode::OpReturn(23));
    my_c.disassemble_chunk("my chunk");

}
