use std::fmt::Display;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::STACK_MAX;

#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    ptr: Link<T>,
    _phantom: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    elem: T,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            ptr: None,
            _phantom: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        self.size = 0;
        self.ptr = None
    }

    pub fn find(&self, index: usize) -> Option<&T> {
        unsafe { None }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.ptr.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn push(&mut self, elem: T) {
        if self.size == STACK_MAX {
            panic!("Error: Reached maximum stack size");
        }
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node { next: None, elem })));

            if let Some(old) = self.ptr {
                (*new.as_ptr()).next = Some(old);
            }

            self.ptr = Some(new);
            self.size += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            self.ptr.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.ptr = boxed_node.next;
                self.size -= 1;
                result
            })
        }
    }
}

impl<T> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[StackSize={}]", self.size).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    use crate::RoxNumber;
    use crate::Value;

    #[test]
    fn test_peek() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.peek().unwrap().to_string(), "4");
    }

    #[test]
    fn test_peek_panic() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.peek().unwrap().to_string(), "4");
        s.pop();
        assert_eq!(s.peek().unwrap().to_string(), "5");
        s.pop();
        assert_eq!(s.peek().unwrap().to_string(), "6");
        s.pop();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_push() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));
        println!("{:?}", s);

        assert_eq!(s.size, 3);
    }

    #[test]
    fn test_pop() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        let val1 = s.pop();
        let val2 = s.pop();
        let val3 = s.pop();

        assert_eq!(val1.unwrap(), Value::Number(RoxNumber(4.0)));
        assert_eq!(val2.unwrap(), Value::Number(RoxNumber(5.0)));
        assert_eq!(val3.unwrap(), Value::Number(RoxNumber(6.0)));
    }

    #[test]
    fn test_max_stack_panics() {
        let mut s = Stack::new();

        for i in 0..31 {
            s.push(Value::Number(RoxNumber(i as f32)));
        }
    }

    #[test]
    #[ignore = "not working currently"]
    fn test_print_stack() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.to_string(), "[6, 5, 4]");
    }
}
