use crate::vm::VM;
use crate::DEBUG_MODE;
use std::io;
use std::io::Write;

pub struct Config {
    pub vm: VM,
    filename: Option<String>,
    pub is_repl: bool,
}

#[derive(Debug)]
pub struct ConfigError {
    msg: String,
}

impl ConfigError {
    fn new(msg: String) -> ConfigError {
        ConfigError { msg }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.msg)
    }
}

impl Config {
    pub fn new(args: &mut std::env::Args) -> Result<Config, ConfigError> {
        if args.len() == 1 {
            Ok(Config {
                vm: VM::new(),
                filename: None,
                is_repl: true,
            })
        } else if args.len() == 2 {
            Ok(Config {
                vm: VM::new(),
                is_repl: false,
                filename: args.next(),
            })
        } else {
            Err(ConfigError::new(String::from("Usage: rox [filename]")))
        }
    }

    pub fn run_file(&mut self) {}

    pub fn repl(&mut self) {
        let mut buffer = String::new();

        loop {
            print!("rox> ");
            io::stdout().flush().unwrap();

            let result = match io::stdin().read_line(&mut buffer) {
                Ok(size) => size,
                Err(_) => continue,
            };

            if DEBUG_MODE {
                print!("Repl read line of length {} -- {}", result, buffer);
            }

            if let Err(val) = self.vm.interpret(&buffer) {
                println!("\n<<<Error in VM interpreter>>>\n\nMessage: {}", val);
            };

            buffer.clear();
        }
    }
}
