use crate::RoxNumber;
use std::ops;

#[derive(Debug)]
pub struct Values {
    pub count: usize,
    pub values: Vec<Value>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(RoxNumber),
    Boolean(bool),
    Nil,
    Error,
}

impl Values {
    pub fn new() -> Values {
        Values {
            count: 0,
            values: vec![],
        }
    }

    /**
     * Writes a value to the values array and returns the index at which it
     * was added for use in the chunk instruction block.
     */
    pub fn write_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.count += 1;
        self.count - 1
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(-num),
            _ => Value::Error,
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        let lhs = match self {
            Value::Number(num) => num,
            _ => return Value::Error,
        };
        let rhs = match rhs {
            Value::Number(num) => num,
            _ => return Value::Error,
        };

        Value::Number(lhs + rhs)
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        let lhs = match self {
            Value::Number(num) => num,
            _ => return Value::Error,
        };
        let rhs = match rhs {
            Value::Number(num) => num,
            _ => return Value::Error,
        };

        Value::Number(lhs - rhs)
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        let lhs = match self {
            Value::Number(num) => num,
            _ => return Value::Error,
        };
        let rhs = match rhs {
            Value::Number(num) => num,
            _ => return Value::Error,
        };

        Value::Number(lhs * rhs)
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        let lhs = match self {
            Value::Number(num) => num,
            _ => return Value::Error,
        };
        let rhs = match rhs {
            Value::Number(num) => num,
            _ => return Value::Error,
        };

        Value::Number(lhs / rhs)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num.to_string()),
            Value::Boolean(b) => write!(f, "{}", b.to_string()),
            Value::Nil => write!(f, "nil"),
            Value::Error => write!(f, "Value<Error>"),
        }
    }
}
