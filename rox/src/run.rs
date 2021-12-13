use crate::vm::VM;
use crate::DEBUG_MODE;
use std::io::Write;
use std::{fs, io};

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
        // skips the first cl arg
        let num_args = args.len();
        args.next();
        if num_args == 1 {
            Ok(Config {
                vm: VM::new(),
                filename: None,
                is_repl: true,
            })
        } else if num_args == 2 {
            Ok(Config {
                vm: VM::new(),
                is_repl: false,
                filename: args.next(),
            })
        } else {
            Err(ConfigError::new(String::from("Usage: rox [filename]")))
        }
    }

    pub fn run_file(&mut self) -> Result<(), ConfigError> {
        // grab filename path if it exists
        let file = match &self.filename {
            Some(val) => val,
            None => return Err(ConfigError::new(String::from("Error retrieving filename."))),
        };

        // read the file contents into string
        let file_contents = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(_) => {
                return Err(ConfigError::new(String::from(format!(
                    "Error reading from file: {}",
                    file,
                ))))
            }
        };

        // interpret the file
        match self.vm.interpret(&file_contents) {
            Err(msg) => return Err(ConfigError::new(msg.message)),
            Ok(outcome) => match outcome {
                crate::InterpretOutcome::InterpretOk => (),
                crate::InterpretOutcome::InterpretCompileError(msg) => {
                    eprintln!("Compiler error with message:\n{}", msg);
                    return Err(ConfigError::new(String::from("Compiler error")));
                }
                crate::InterpretOutcome::InterpretRuntimeError(msg) => {
                    eprintln!("Runtime error with message:\n{}", msg);
                    return Err(ConfigError::new(String::from("Runtime error")));
                }
            },
        };
        Ok(())
    }

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
