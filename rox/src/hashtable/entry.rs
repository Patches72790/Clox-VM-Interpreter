use crate::value::Value;
use crate::RoxString;

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub key: RoxString,
    pub value: Value,
}

