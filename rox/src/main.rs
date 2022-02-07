use rox::Config;
use std::env::args;

fn main() {
    let mut config = Config::new(&mut args()).unwrap_or_else(|err| {
        eprintln!(
            "\n<<<Error with command line arguments>>>\n\nExiting with message:\n{}",
            err
        );
        std::process::exit(1);
    });

    if config.is_repl {
        config.repl();
    } else {
        config.run_file().unwrap_or_else(|msg| {
            eprintln!("\n<<<Error in Rox interpreter>>>\n\nMessage: {}", msg);
        });
    }
}
