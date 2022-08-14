use rox::Config;
use std::env::args;

macro_rules! make_rox_test {
    ($fn_name:ident, $filename:literal) => {
        #[test]
        fn $fn_name() {
            let config = Config::new(&mut args()).unwrap();

            match config.run_file_with_filename($filename) {
                Err(msg) => panic!("{}", msg),
                _ => (),
            };
        }
    };
}

macro_rules! make_rox_test_panic {
    ($fn_name:ident, $filename:literal) => {
        #[test]
        #[should_panic]
        fn $fn_name() {
            let config = Config::new(&mut args()).unwrap();

            match config.run_file_with_filename($filename) {
                Err(_) => panic!("Correctly panicked in test for {}", $filename),
                _ => (),
            };
        }
    };
}

make_rox_test!(test_basic_expression, "rox_tests/basic_expr_test.rox");
make_rox_test!(test_local_var_scope, "rox_tests/local_var_scope.rox");
make_rox_test_panic!(
    test_local_var_reassignment,
    "rox_tests/local_var_reassign.rox"
);
make_rox_test!(test_basic_if_statement, "rox_tests/if_statement.rox");
make_rox_test!(test_if_else_statement, "rox_tests/if_else_statement.rox");
make_rox_test!(test_if_statement_scope, "rox_tests/if_statement_scope.rox");
make_rox_test!(test_logical_expr, "rox_tests/logical_expr.rox");
make_rox_test!(test_while_loop, "rox_tests/while_loop.rox");
make_rox_test!(test_for_loop, "rox_tests/for_loop.rox");
