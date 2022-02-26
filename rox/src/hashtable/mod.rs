mod entry;

use crate::value::Value;
use crate::RoxString;
pub use entry::Entry;
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

const INITIAL_TABLE_CAPACITY: usize = 8;
static LOAD_FACTOR: f32 = 0.75;

pub trait RoxMap<K = RoxString, V = Value, E = Entry> {
    fn get(&self, key: &K) -> Option<E>;

    fn set(&mut self, key: &K, value: &V);

    fn contains(&self, key: &K) -> bool;

    fn remove(&mut self, key: &K) -> Option<E>;
}

impl RoxMap for Table {
    fn get(&self, key: &RoxString) -> Option<Entry> {
        let bucket = self.hash_key(&key);

        unsafe {
            let mut try_index = bucket;
            loop {
                let try_ptr = self.table.as_ptr().add(try_index.try_into().unwrap());
                let maybe_value = ptr::read(try_ptr);

                match maybe_value {
                    Some(val) => {
                        if val.key == *key {
                            break Some(val);
                        }
                    }
                    None => break None,
                }

                try_index = (try_index + 1) % (self.capacity as u64);
            }
        }
    }

    fn set(&mut self, key: &RoxString, value: &Value) {
        if self.length as f32 >= self.load_factor() {
            self.grow();
        }

        let bucket = self.hash_key(&key);
        println!(
            "Bucket for key {key} is {bucket} with capacity {}",
            self.capacity
        );

        let new_entry = Entry {
            key: key.clone(),
            value: value.clone(),
        };

        // TODO! need to implement linear probing for insertion to deal
        // with duplicate hashed keys
        unsafe {
            let mut try_index = bucket;
            loop {
                let try_ptr = self.table.as_ptr().add(try_index.try_into().unwrap());
                let check_for_zero_ptr = NonNull::new(try_ptr)
                    .expect("Error creating Non-Null pointer from hash table pointer.")
                    .cast::<u8>();

                if *check_for_zero_ptr.as_ptr() == 0 {
                    ptr::write(try_ptr, Some(new_entry));
                    break;
                }

                try_index = (try_index + 1) % (self.capacity as u64);
            }
        }

        self.length += 1;
    }

    fn contains(&self, key: &RoxString) -> bool {
        todo!()
    }

    fn remove(&mut self, key: &RoxString) -> Option<Entry> {
        todo!()
    }
}

pub struct Table {
    table: NonNull<Option<Entry>>,
    capacity: usize,
    length: usize,
    _marker: PhantomData<Option<Entry>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            table: NonNull::dangling(),
            capacity: 0,
            length: 0,
            _marker: PhantomData,
        }
    }

    fn size(&self) -> usize {
        self.length
    }

    fn hash_key(&self, key: &RoxString) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.clone().as_bytes());
        let hashed_key = hasher.finish();

        hashed_key % (self.capacity as u64)
    }

    fn load_factor(&self) -> f32 {
        self.capacity as f32 * LOAD_FACTOR
    }

    fn grow(&mut self) {
        // this sets the initial capacity if the
        // table is empty
        let (new_capacity, new_layout) = if self.capacity == 0 {
            (
                INITIAL_TABLE_CAPACITY,
                Layout::array::<Option<Entry>>(INITIAL_TABLE_CAPACITY)
                    .expect("Error initializing layout for table"),
            )
        } else {
            let new_capacity = 2 * self.capacity;
            let new_layout = Layout::array::<Option<Entry>>(new_capacity)
                .expect("Error increasing size of layout for table");
            (new_capacity, new_layout)
        };

        // this allocates memory for the new layout/capacity
        // always allocate the memory to zero
        let new_ptr = unsafe { alloc_zeroed(new_layout) };

        // get the pointer for the newly allocated memory here
        self.table = match NonNull::new(new_ptr as *mut Option<Entry>) {
            Some(p) => {
                if self.capacity != 0 {
                    self.rehash_entries(p.as_ptr(), new_capacity);
                }

                p
            }
            None => std::alloc::handle_alloc_error(new_layout),
        };

        self.capacity = new_capacity;
    }

    /// This method takes the entries from the previous
    /// hash table and reinserts them into the newly
    /// allocated array with the new capacity.
    fn rehash_entries(&mut self, new_ptr: *mut Option<Entry>, new_capacity: usize) {
        todo!()
    }

    fn shrink(&mut self) {
        todo!()
    }

    /// Not safe to use since some of the entries may be
    /// uninitialized or filled with garbage values...
    fn entries(&self) -> &[Option<Entry>] {
        todo!()
        //unsafe { std::slice::from_raw_parts(self.table.as_ptr(), self.capacity) }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<Option<Entry>>();

        if self.capacity != 0 && elem_size != 0 {
            unsafe {
                dealloc(
                    self.table.as_ptr() as *mut u8,
                    Layout::array::<Option<Entry>>(self.capacity).unwrap(),
                );
            }
        }
    }
}

unsafe impl Send for Table {}
unsafe impl Sync for Table {}

#[cfg(test)]
mod tests {
    use crate::RoxNumber;

    use super::*;

    #[test]
    fn test_new_table() {
        let mut table = Table::new();

        table.grow();
        assert_eq!(table.capacity, 8);
        assert!(table.table != NonNull::dangling());
        assert_eq!(table.size(), 0);
    }

    #[test]
    fn test_basic_table_get_and_set() {
        let mut table = Table::new();
        let key = RoxString::new("Hello");
        let key2 = RoxString::new("adfasdfasdfafadf");
        let key3 = RoxString::new("what a world we live in?@?!");
        let value1 = Value::Number(RoxNumber(45.0));
        let value2 = Value::Number(RoxNumber(90.0));
        let value3 = Value::Number(RoxNumber(180.0));
        let entry1 = Entry {
            key: RoxString::new("Hello"),
            value: Value::Number(RoxNumber(45.0)),
        };
        let entry2 = Entry {
            key: RoxString::new("adfasdfasdfafadf"),
            value: Value::Number(RoxNumber(90.0)),
        };
        let entry3 = Entry {
            key: RoxString::new("what a world we live in?@?!"),
            value: Value::Number(RoxNumber(180.0)),
        };

        table.set(&key, &value1);
        table.set(&key2, &value2);
        table.set(&key3, &value3);

        assert_eq!(table.get(&key), Some(entry1));
        assert_eq!(table.get(&key2), Some(entry2));
        assert_eq!(table.get(&key3), Some(entry3));
        assert_eq!(table.size(), 3);
    }

    #[test]
    fn test_basic_contains_key() {
        let mut table = Table::new();
        let key = RoxString::new("Hello");
        let value = Value::Number(RoxNumber(45.0));
        let key2 = RoxString::new("adfasdfasdfafadf");
        let value2 = Value::Number(RoxNumber(90.0));
        let key3 = RoxString::new("what a world we live in?@?!");
        let value3 = Value::Number(RoxNumber(180.0));

        table.set(&key, &value);
        table.set(&key2, &value2);
        table.set(&key3, &value3);

        assert!(table.contains(&key));
        assert!(table.contains(&key2));
        assert!(table.contains(&key3));
    }

    #[test]
    fn test_basic_remove_key() {
        let mut table = Table::new();
        let key = RoxString::new("Hello");
        let value = Value::Number(RoxNumber(45.0));
        let key2 = RoxString::new("adfasdfasdfafadf");
        let value2 = Value::Number(RoxNumber(90.0));
        let key3 = RoxString::new("what a world we live in?@?!");
        let value3 = Value::Number(RoxNumber(180.0));

        table.set(&key, &value);
        table.set(&key2, &value2);
        table.set(&key3, &value3);

        table.remove(&key);
        table.remove(&key2);
        table.remove(&key3);

        assert!(!table.contains(&key));
        assert!(!table.contains(&key2));
        assert!(!table.contains(&key3));
    }
}
