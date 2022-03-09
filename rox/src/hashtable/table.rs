use crate::hashtable::entry::Entry;
use crate::hashtable::map::RoxMap;
use std::collections::HashMap;
use std::hash::Hash;

impl<K, V> RoxMap<K, V> for StdTable<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
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
}
