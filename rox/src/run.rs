use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::repl::ScreenClear;
use crate::vm::VM;
use crate::InterpretError;
use crate::Repl;
use crate::DEBUG_MODE;
use std::collections::VecDeque;
use std::io::stdout;
use std::io::Write;
use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    vm: VM,
    filename: Option<String>,
    pub is_repl: bool,
}

#[derive(Debug)]
pub struct ConfigError {
    msg: String,
}

impl From<InterpretError> for ConfigError {
    fn from(error: InterpretError) -> Self {
        let msg = match error {
            InterpretError::CompileError(msg) => msg,
            InterpretError::RuntimeError(msg) => msg,
        };
        Self { msg }
    }
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

    pub fn run_file_with_filename(&mut self, pathname: &str) -> Result<(), ConfigError> {
        // read the file contents into string
        let file_contents = match fs::read_to_string(pathname) {
            Ok(content) => content,
            Err(msg) => {
                return Err(ConfigError::new(format!(
                    "Error reading from file {} with msg: {}",
                    pathname, msg
                )))
            }
        };

        if DEBUG_MODE {
            println!("Read contents of file:\n\n{file_contents}");
        }

        // interpret the file
        self.vm.interpret(&file_contents)?;
        Ok(())
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
                return Err(ConfigError::new(format!(
                    "Error reading from file: {}",
                    file,
                )))
            }
        };

        if DEBUG_MODE {
            println!("Read contents of file:\n{file_contents}");
        }

        // interpret the file
        self.vm.interpret(&file_contents)?;
        Ok(())
    }

    pub fn repl(&mut self) {
        let mut repl = Repl::new();
        repl.screen_update(crate::repl::ScreenClear::All).unwrap();

        loop {
            repl.flush().unwrap();
            repl.write("rox> ").unwrap();
            repl.flush().unwrap();

            for c in io::stdin().keys() {
                match c.unwrap_or_else(|e| panic!("Error reading key {e}")) {
                    termion::event::Key::Ctrl('l') => {
                        repl.screen_update(crate::repl::ScreenClear::All).unwrap()
                    }
                    termion::event::Key::Ctrl('q' | 'd') => return,
                    termion::event::Key::Char('\n') => {
                        repl.screen_update(crate::repl::ScreenClear::NewLine)
                            .unwrap();
                        repl.write("\n").unwrap();
                        break;
                    }

                    termion::event::Key::Char(c) => repl
                        .add_char(c)
                        .unwrap_or_else(|e| panic!("Error adding new char: {e}")),
                    termion::event::Key::Backspace => {
                        let current = repl
                            .remove_char()
                            .unwrap_or_else(|e| panic!("Error removing char: {e}"));
                        repl.screen_update(ScreenClear::AfterCursor)
                            .expect("Error updating screen");

                        repl.write("rox> ").unwrap();
                        repl.write(current.as_str()).unwrap();
                    }
                    termion::event::Key::Up => todo!(),
                    termion::event::Key::Down => todo!(),
                    _ => todo!(),
                }

                repl.flush().unwrap();
            }

            let input = repl
                .current()
                .unwrap_or_else(|| panic!("Error getting input at current history"));

            if let Err(val) = self.vm.interpret(input) {
                panic!("Error: {val}");
                //                screen_idx += 1;
                //                writeln!(stdout, "{}", termion::cursor::Goto(1, screen_idx)).unwrap();
                //                writeln!(stdout, "{}", val).unwrap();
                //                screen_idx += 1;
                //                writeln!(stdout, "{}", termion::cursor::Goto(1, screen_idx)).unwrap();
            };

            self.vm.reset();
        }
    }
    /*
    pub fn repl(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut history = VecDeque::new();
        let mut history_idx = 0usize;
        let mut screen_idx = 1;

        write!(
            stdout,
            r#"{}{}"#,
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        loop {
            stdout.flush().unwrap();
            write!(stdout, "rox> ").unwrap();
            stdout.flush().unwrap();

            history.push_front(String::new());

            for c in io::stdin().keys() {
                match c.unwrap_or_else(|e| panic!("Error reading key {e}")) {
                    termion::event::Key::Ctrl('l') => {
                        screen_idx = 1;
                        write!(
                            stdout,
                            r#"{}{}"#,
                            termion::cursor::Goto(1, screen_idx),
                            termion::clear::AfterCursor
                        )
                        .unwrap();
                        write!(stdout, "rox> ").unwrap();
                    }
                    termion::event::Key::Ctrl('q' | 'd') => return,
                    termion::event::Key::Char('\n') => {
                        screen_idx += 1;
                        write!(
                            stdout,
                            r#"{}{}"#,
                            termion::cursor::Goto(1, screen_idx),
                            termion::clear::AfterCursor
                        )
                        .unwrap();
                        writeln!(stdout).unwrap();
                        break;
                    }

                    termion::event::Key::Backspace => {
                        if let Some(current) = history.get_mut(history_idx) {
                            current.pop();

                            write!(
                                stdout,
                                r#"{}{}"#,
                                termion::cursor::Goto(1, screen_idx),
                                termion::clear::AfterCursor,
                            )
                            .unwrap();

                            write!(stdout, "rox> {}", current).unwrap();
                        }
                    }
                    termion::event::Key::Char(c) => {
                        if let Some(current) = history.get_mut(history_idx) {
                            current.push(c);

                            write!(stdout, "{}", c).unwrap();
                        }
                    }
                    _ => (),
                }

                stdout.flush().unwrap();
            }

            let input = history.get(history_idx).unwrap_or_else(|| {
                panic!("Error getting input at current history index {history_idx}")
            });

            if let Err(val) = self.vm.interpret(input) {
                screen_idx += 1;
                writeln!(stdout, "{}", termion::cursor::Goto(1, screen_idx)).unwrap();
                writeln!(stdout, "{}", val).unwrap();
                screen_idx += 1;
                writeln!(stdout, "{}", termion::cursor::Goto(1, screen_idx)).unwrap();
            };

            self.vm.reset();
        }
    }
    */
}
