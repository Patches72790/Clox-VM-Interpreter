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

#[test]
fn test_local_var_scope() {
    let config = Config::new(&mut args()).unwrap();

    match config.run_file_with_filename("rox_tests/local_var_scope.rox") {
        Err(msg) => panic!("{}", msg),
        _ => (),
    };
}

#[test]
#[should_panic]
fn test_local_var_reassignment() {
    let config = Config::new(&mut args()).unwrap();

    match config.run_file_with_filename("rox_tests/local_var_reassign.rox") {
        Err(_) => panic!("Correctly failed reassignment test"),
        _ => (),
    };
}
