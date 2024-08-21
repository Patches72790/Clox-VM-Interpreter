use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use crate::STACK_MAX;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: VecDeque<T>,
    size: usize,
}

impl<T> Default for Stack<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T>
where
    T: Clone + Default,
{
    pub fn new() -> Self {
        Self {
            items: vec![Default::default(); STACK_MAX].into(),
            size: 0,
        }
    }

    pub fn reset(&mut self) {
        self.items = vec![].into();
        self.size = 0;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, elem: T) {
        self.items.push_front(elem);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.items.pop_front() {
            Some(val) => {
                self.size -= 1;
                Some(val)
            }
            _ => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn get_and_push_local(&mut self, index: usize) -> Result<(), String> {
        if index >= self.size() {
            return Err("Cannot get local at index beyond stack size".into());
        }

        let local = &self.items[index];
        self.items.push_front(local.clone());
        self.size += 1;

        Ok(())
    }

    pub fn set_local(&mut self, index: usize) -> Result<(), String> {
        match self.peek() {
            Some(local) => {
                self.items[index] = local.clone();
                Ok(())
            }
            None => Err("Error cannot set local in empty stack".into()),
        }
    }
}

impl<T> Display for Stack<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{:?}",
                self.items.iter().take(self.size).collect::<Vec<_>>()
            )
            .as_str(),
        )
    }
}
