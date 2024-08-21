use crate::{ObjectType, RoxNumber, RoxObject};
use std::{fmt::Write, ops};

#[derive(Debug, Default, Clone)]
pub struct Values {
    pub count: usize,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, Eq, Default)]
pub enum Value {
    #[default]
    Nil,
    Number(RoxNumber),
    Boolean(bool),
    Object(RoxObject),
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
    pub fn write_value(&mut self, value: Value) -> (usize, &mut Value) {
        self.values.push(value);
        self.count += 1;
        let index = self.count - 1;
        let value_ref = self.values.get_mut(index).unwrap();

        (index, value_ref)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Value::Number(self_num) => match other {
                Value::Number(other_num) => self_num.partial_cmp(other_num),
                _ => None,
            },
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Number(a_num) => match other {
                Value::Number(b_num) => a_num == b_num,
                _ => false,
            },
            Value::Boolean(a_bool) => match other {
                Value::Boolean(b_bool) => a_bool == b_bool,
                _ => false,
            },
            Value::Nil => matches!(other, Value::Nil),
            Value::Object(obj) => match &obj.object_type {
                ObjectType::ObjString(string_one) => match other {
                    Value::Object(obj_two) => match &obj_two.object_type {
                        ObjectType::ObjString(string_two) => string_one == string_two,
                    },
                    _ => false,
                },
            },
            _ => false,
        }
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
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Object(obj) => match &obj.object_type {
                ObjectType::ObjString(s) => write!(f, "\"{}\"", s),
                //_ => unimplemented!("Unimplemented object type display!"),
            },
            Value::Error => write!(f, "Value<Error>"),
        }
    }
}
