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
pub struct Entry<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> Entry<K, V>
where
    K: Clone,
    V: Clone,
{
    pub fn new_full(key: &K, value: &V) -> Entry<K, V> {
        Entry {
            key: key.clone(),
            value: value.clone(),
        }
    }
}
