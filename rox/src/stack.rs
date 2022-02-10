use crate::Value;
use crate::STACK_MAX;

pub struct Stack {
    pub values: Vec<Value>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        if self.values.len() == STACK_MAX as usize {
            panic!("Error: Attempting to push value beyond maximum stack size!");
        }
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, &'static str> {
        match self.values.pop() {
            Some(val) => Ok(val),
            None => Err("Cannot pop from empty VM stack!"),
        }
        //self.values.pop().expect("Cannot pop from empty VM stack!")
    }

    pub fn peek(&self) -> Option<Value> {
        self.values.iter().rev().peekable().peek()
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        for (i, val) in self.values.iter().enumerate() {
            s.push_str(&(val.to_string()));
            if i < self.values.len() - 1 {
                s.push_str(", ");
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
    fn test_push() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        assert_eq!(s.to_string(), "[6, 5, 4]");
    }

    #[test]
    #[should_panic]
    fn test_pop() {
        let mut s = Stack::new();
        s.push(Value::Number(RoxNumber(6.0)));
        s.push(Value::Number(RoxNumber(5.0)));
        s.push(Value::Number(RoxNumber(4.0)));

        s.pop();
        s.pop();
        s.pop();
        s.pop();
    }

    #[test]
    #[should_panic]
    fn test_max_stack_panics() {
        let mut s = Stack::new();

        for i in 0..STACK_MAX + 1 {
            s.push(Value::Number(RoxNumber(i as f32)));
        }
    }
}
