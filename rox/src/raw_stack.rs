use crate::Value;
use crate::STACK_MAX;

#[derive(Debug)]
pub struct RawStack {
    pub values: [Option<Value>; STACK_MAX],
    pub size: usize,
    pub stack_ptr: *mut Option<Value>,
}

impl RawStack {
    pub fn new() -> RawStack {
        let mut values = [(); STACK_MAX].map(|_| Option::<Value>::default());
        let stack_ptr = values.as_mut_ptr();
        RawStack {
            values,
            size: 0,
            stack_ptr,
        }
    }

    pub fn reset_stack(&mut self) {
        self.size = 0;
        self.stack_ptr = self.values.as_mut_ptr();
    }

    pub fn peek(&self, distance: usize) -> Result<Value, &'static str> {
        unsafe {
            let d = (self.size - distance) as isize;
            if d < 0 {
                panic!("Cannot peek beyond bottom of stack!");
            }

            let val = &*self.stack_ptr.offset(-1 - distance as isize);
            let val = val.as_ref().expect("Error peeking value from stack");
            Ok(val.clone())
        }
    }

    pub fn push(&mut self, value: Value) {
        unsafe {
            if self.size == STACK_MAX {
                panic!("Cannot push beyond maximum stack size of {}", STACK_MAX);
            }

            *self.stack_ptr = Some(value);
            self.size += 1;
            self.stack_ptr = self.stack_ptr.offset(1);
        }
    }

    pub fn pop(&mut self) -> Result<Value, &'static str> {
        unsafe {
            if self.size == 0 {
                return Err("Cannot pop from empty VM stack!");
            }
            let new_ptr = self.stack_ptr.offset(-1);
            let val = &*new_ptr;
            self.stack_ptr = new_ptr;
            self.size -= 1;
            match val {
                Some(val) => Ok(val.clone()),
                None => Err("Cannot pop from empty VM stack!"),
            }
        }
    }
}

impl std::fmt::Display for RawStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        // TODO! Need to implement this to use stack ptr instead of array
        for i in 0..self.size {
            if let Some(val) = &self.values[i] {
                s.push_str(&(val.to_string()));
                if i < self.size - 1 {
                    s.push_str(", ");
                }
            }
        }
        s.push_str("]");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RoxNumber;

    #[test]
    fn test_peek() {
        let mut s = RawStack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.peek(0).ok().unwrap().to_string(), "4");
        assert_eq!(s.peek(1).ok().unwrap().to_string(), "5");
        assert_eq!(s.peek(2).ok().unwrap().to_string(), "6");
    }

    #[test]
    #[should_panic]
    fn test_peek_panic() {
        let mut s = RawStack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.peek(3).ok().unwrap().to_string(), "4");
        assert_eq!(s.peek(4).ok().unwrap().to_string(), "5");
        assert_eq!(s.peek(5).ok().unwrap().to_string(), "6");
    }

    #[test]
    fn test_push() {
        let mut s = RawStack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));
        println!("{:?}", s);

        assert_eq!(s.size, 3);
    }

    #[test]
    fn test_pop() -> Result<(), &'static str> {
        let mut s = RawStack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        let val1 = s.pop()?;
        let val2 = s.pop()?;
        let val3 = s.pop()?;

        assert_eq!(val1, Value::Number(RoxNumber(4.0)));
        assert_eq!(val2, Value::Number(RoxNumber(5.0)));
        assert_eq!(val3, Value::Number(RoxNumber(6.0)));

        if let Ok(_) = s.pop() {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_max_stack_panics() {
        let mut s = RawStack::new();

        for i in 0..STACK_MAX + 1 {
            s.push(Value::Number(RoxNumber(i as f32)));
        }
    }

    #[test]
    #[ignore = "not working currently"]
    fn test_print_stack() {
        let mut s = RawStack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.to_string(), "[6, 5, 4]");
    }
}
