use rox::Config;
use std::env::args;

fn main() {
    let mut config = match Config::new(&mut args()) {
        Ok(cfg) => cfg,
        Err(msg) => {
            println!(
                "\n<<<Error with command line arguments>>>\n\nExiting with message:\n{}",
                msg
            );
            std::process::exit(1);
        }
    };

    if config.is_repl {
        config.repl();
    } else {
        config.run_file();
    }

    //    let mut vm = VM::new();
    //    // adding constant value
    //    vm.chunk.add_constant(Value::Number(45.0), 1);
    //    vm.chunk.add_constant(Value::Number(69.0), 6);
    //    vm.chunk.write_chunk(OpCode::OpAdd, 1);
    //    vm.chunk.write_chunk(OpCode::OpNegate, 4);
    //    vm.chunk.write_chunk(OpCode::OpReturn(12), 4);
    //
    //    loop {
    //        if let Err(val) = vm.interpret() {
    //            println!(
    //                "\n<<<Error in VM interpreter>>>\n\nExiting with message: {}",
    //                val
    //            );
    //            break;
    //        }
    //    }
}
