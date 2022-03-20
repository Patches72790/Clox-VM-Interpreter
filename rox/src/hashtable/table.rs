use crate::hashtable::entry::Entry;
use crate::hashtable::map::RoxMap;
use crate::DEBUG_MODE;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

impl<K, V> RoxMap<K, V> for StdTable<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Debug,
{
    fn get(&self, key: &K) -> Option<&V> {
        match self.inner_table.get(&key) {
            Some(entry) => Some(&entry.value),
            _ => None,
        }
    }

    fn set(&mut self, key: &K, value: &V) -> bool {
        match self
            .inner_table
            .insert(key.clone(), Entry::new_full(key, value))
        {
            Some(_) => true,
            None => false,
        }
    }

    /// Sets the value at key if it already exists
    /// in the map. If the key doesn't already exist, then it returns false
    /// and does not set a new key-value pair.
    fn get_and_set(&mut self, key: &K, value: &V) -> bool {
        match self.inner_table.get(key) {
            Some(s) => {
                if DEBUG_MODE {
                    println!("Overwriting string {:?}", s);
                }

                self.set(key, value);
                true
            }
            None => false,
        }
    }

    fn remove(&mut self, key: K) -> Option<V> {
        match self.inner_table.remove(&key) {
            Some(entry) => Some(entry.value),
            _ => None,
        }
    }

    fn contains(&self, key: K) -> bool {
        self.inner_table.contains_key(&key)
    }
}

#[derive(Debug)]
pub struct StdTable<K, V> {
    inner_table: HashMap<K, Entry<K, V>>,
}

impl<K, V> StdTable<K, V> {
    pub fn new() -> StdTable<K, V> {
        let inner_table: HashMap<K, Entry<K, V>> = HashMap::new();
        StdTable { inner_table }
    }

    pub fn reset(&mut self) {
        self.inner_table.drain();
    }
}
