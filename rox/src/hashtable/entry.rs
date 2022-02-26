use crate::value::Value;
use crate::RoxString;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub key: RoxString,
    pub value: Value,
}

impl Entry {
    pub fn new_nil_entry() -> Entry {
        Entry::new(&RoxString::new(""), &Value::Nil)
    }

    pub fn new(key: &RoxString, value: &Value) -> Entry {
        Entry {
            key: key.clone(),
            value: value.clone(),
        }
    }

    pub fn is_nil(&self) -> bool {
        match self.value {
            Value::Nil => true,
            _ => false,
        }
    }
}
