use crate::value::Value;
use crate::RoxString;

///
/// TODO! Need to think about how to represent
/// tombstones (i.e. deleted entries).
/// Maybe keep a u8 and use a mask to mark states:
///
/// [1111_1111] == Allocated
/// [0000_0000] == Not-allocated
/// [0000_0001] == Deleted (tombstone)
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    state: u8,
    pub key: RoxString,
    pub value: Value,
}

const ENTRY_FULL: u8 = 0b0000_0001;
const ENTRY_EMPTY: u8 = 0b0000_0010;
const ENTRY_DELETED: u8 = 0b0000_0100;

impl Entry {
    pub fn new_full(key: &RoxString, value: &Value) -> Entry {
        Entry {
            state: ENTRY_FULL,
            key: key.clone(),
            value: value.clone(),
        }
    }

    pub fn new_empty() -> Entry {
        Entry {
            state: ENTRY_EMPTY,
            key: RoxString::new(""),
            value: Value::Nil,
        }
    }

    pub fn set_deleted(&mut self) {
        self.state = ENTRY_DELETED;
    }

    pub fn is_deleted(&self) -> bool {
        self.state & ENTRY_DELETED != 0
    }

    pub fn is_full(&self) -> bool {
        self.state & ENTRY_FULL != 0
    }

    pub fn is_empty(&self) -> bool {
        self.state & ENTRY_EMPTY != 0
    }
}
