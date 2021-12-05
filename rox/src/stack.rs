use crate::Value;
use crate::STACK_MAX;

pub struct Stack {
    values: Vec<Value>,
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

    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("Cannot pop from empty VM stack!")
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

    #[test]
    fn test_push() {
        let mut s = Stack::new();
        s.push(Value::Number(6.0));
        s.push(Value::Number(5.0));
        s.push(Value::Number(4.0));

        assert_eq!(s.to_string(), "[6, 5, 4]");
    }

    #[test]
    #[should_panic]
    fn test_pop() {
        let mut s = Stack::new();
        s.push(Value::Number(6.0));
        s.push(Value::Number(5.0));
        s.push(Value::Number(4.0));

        s.pop();
        s.pop();
        s.pop();
        s.pop();
    }
}
