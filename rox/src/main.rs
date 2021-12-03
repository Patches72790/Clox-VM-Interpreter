use rox::{OpCode, Value, VM};

fn main() {
    let mut vm = VM::new();
    // adding some return opcodes
    vm.chunk.write_chunk(OpCode::OpReturn(8), 1);
    vm.chunk.write_chunk(OpCode::OpReturn(22), 1);
    vm.chunk.write_chunk(OpCode::OpReturn(55), 1);

    // adding constant value
    let index = vm.chunk.add_constant(Value::Number(45.0));
    vm.chunk.write_chunk(OpCode::OpConstant(index), 1);

    vm.chunk.write_chunk(OpCode::OpReturn(12), 4);

    // TODO -- Think of better way of adding in blank lines for lines array?
    let index = vm.chunk.add_constant(Value::Number(69.0));
    vm.chunk.write_chunk(OpCode::OpConstant(index), 6);
    // debug print instructions
    vm.chunk.disassemble_chunk("my chunk");
}
