use rox::Config;
use std::env::args;

#[test]
fn test_basic_expression() {
    let config = Config::new(&mut args()).unwrap();

    match config.run_file_with_filename("rox_tests/basic_expr_test.rox") {
        Err(msg) => panic!("{}", msg),
        _ => (),
    };
}
