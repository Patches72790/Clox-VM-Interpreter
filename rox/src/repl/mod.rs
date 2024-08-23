use std::{
    collections::VecDeque,
    io::{stdout, Stdout, Write},
};

use termion::raw::{IntoRawMode, RawTerminal};

pub struct Repl {
    history_idx: usize,
    history: VecDeque<String>,
    screen_idx: u16,
    out: RawTerminal<Stdout>,
}

pub enum ScreenClear {
    All,
    AfterCursor,
    NewLine,
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

impl Repl {
    pub fn new() -> Self {
        Self {
            history_idx: 0,
            history: VecDeque::from(["".to_string()]),
            screen_idx: 1,
            out: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn current(&self) -> Option<&String> {
        self.history.get(self.history_idx)
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn remove_char(&mut self) -> Result<String, String> {
        if let Some(current) = self.history.get_mut(self.history_idx) {
            current.pop();
            Ok(current.to_string())
        } else {
            Err("Error removing char".to_string())
        }
    }

    pub fn add_char(&mut self, char: char) -> Result<(), String> {
        if let Some(current) = self.history.get_mut(self.history_idx) {
            current.push(char);
            Ok(self
                .write(&char.to_string())
                .map_err(|e| format!("Error writing new char {e}"))?)
        } else {
            Err("Error adding char".to_string())
        }
    }

    pub fn history(&self) {
        todo!("History update!")
    }
    /*
    termion::event::Key::Up => {
        history_idx += 1 % history.len();
        let n = history.get(history_idx);
        if let Some(s) = n {
            write!(stdout, "{}", s).unwrap();
        }
        //write!(stdout, "{}", history.get(history_idx).unwrap_unchecked()).unwrap();
    }
    termion::event::Key::Down => {
        history_idx = history_idx.saturating_sub(1);
        let n = history.get(history_idx);
        if let Some(s) = n {
            write!(stdout, "{}", s).unwrap();
        }
    }
    */

    pub fn screen_update(&mut self, wheree: ScreenClear) -> Result<(), std::io::Error> {
        match wheree {
            ScreenClear::All => write!(
                self.out,
                r#"{}{}"#,
                termion::cursor::Goto(1, self.screen_idx),
                termion::clear::All
            ),
            ScreenClear::AfterCursor => write!(
                self.out,
                r#"{}{}"#,
                termion::cursor::Goto(1, self.screen_idx),
                termion::clear::AfterCursor
            ),
            ScreenClear::NewLine => write!(
                self.out,
                r#"{}{}"#,
                termion::cursor::Goto(1, self.screen_idx),
                termion::clear::UntilNewline
            ),
        }
    }

    pub fn write(&mut self, msg: &str) -> Result<(), std::io::Error> {
        self.screen_idx += 1;
        write!(self.out, "{}", msg)
    }
}
