use rox::{OpCode, Value, VM};

fn main() {
    let mut vm = VM::new();
    // adding constant value
    vm.chunk.add_constant(Value::Number(45.0), 1);
    vm.chunk.add_constant(Value::Number(69.0), 6);
    vm.chunk.write_chunk(OpCode::OpAdd, 1);
    vm.chunk.write_chunk(OpCode::OpNegate, 4);
    vm.chunk.write_chunk(OpCode::OpReturn(12), 4);

    loop {
        if let Err(val) = vm.interpret() {
            println!(
                "\n<<<Error in VM interpreter>>>\n\nExiting with message: {}",
                val
            );
            break;
        }
    }
}
