use crate::hashtable::entry::Entry;
use crate::{hashtable::map::RoxMap, RoxString, Value};
use std::collections::HashMap;
use std::rc::Rc;

impl RoxMap for StdTable {
    fn get(&self, key: Rc<RoxString>) -> Option<&Entry> {
        self.inner_table.get(&key)
    }

    fn set(&mut self, key: Rc<RoxString>, value: &Value) {
        let entry = Entry::new_full(&key, value);
        self.inner_table.insert(key, entry);
    }

    fn remove(&mut self, key: Rc<RoxString>) -> Option<Entry> {
        self.inner_table.remove(&key)
    }

    fn contains(&self, key: Rc<RoxString>) -> bool {
        self.inner_table.contains_key(&key)
    }
}

pub struct StdTable {
    inner_table: HashMap<Rc<RoxString>, Entry>,
}

impl StdTable {
    pub fn new() -> StdTable {
        StdTable {
            inner_table: HashMap::new()
        }
    }
}
