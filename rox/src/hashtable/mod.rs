mod entry;

use crate::value::Value;
use crate::RoxString;
pub use entry::Entry;
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr::{self, NonNull};

const INITIAL_TABLE_CAPACITY: usize = 8;
static LOAD_FACTOR: f32 = 0.5;

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

                if maybe_value.is_nil() {
                    break None;
                } else if maybe_value.key == *key {
                    break Some(maybe_value);
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

        let new_entry = Entry::new(&key, &value);

        // linear probing to find open bucket for new entry
        unsafe {
            let mut try_index = bucket;
            loop {
                let try_ptr = self.table.as_ptr().add(try_index.try_into().unwrap());
                let maybe_value = ptr::read(try_ptr);

                if maybe_value.is_nil() {
                    ptr::write(try_ptr, new_entry);
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
    table: NonNull<Entry>,
    capacity: usize,
    length: usize,
    _marker: PhantomData<Entry>,
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

    fn hash_key_with_capacity(&self, key: &RoxString, capacity: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.clone().as_bytes());
        let hashed_key = hasher.finish();

        hashed_key % (capacity as u64)
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
                Layout::array::<Entry>(INITIAL_TABLE_CAPACITY)
                    .expect("Error initializing layout for table"),
            )
        } else {
            let new_capacity = 2 * self.capacity;
            let new_layout = Layout::array::<Entry>(new_capacity)
                .expect("Error increasing size of layout for table");
            (new_capacity, new_layout)
        };

        // this allocates memory for the new layout/capacity
        // always allocate the memory to zero
        let new_ptr = unsafe { alloc_zeroed(new_layout) };

        // get the pointer for the newly allocated memory here
        self.table = match NonNull::new(new_ptr as *mut Entry) {
            Some(p) => {
                // initialize values to None
                unsafe {
                    for idx in 0..new_capacity {
                        let ptr = p.as_ptr();
                        *ptr.add(idx) = Entry::new_nil_entry();
                    }
                    println!("Initialized table to Nil entries");
                }

                if self.capacity != 0 {
                    println!("Rehashing table to {}", new_capacity);
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
    fn rehash_entries(&self, new_ptr: *mut Entry, new_capacity: usize) {
        unsafe {
            let entries = self.deref();
            for entry in entries {
                if !entry.is_nil() {
                    let new_bucket = self.hash_key_with_capacity(&entry.key, new_capacity);
                    let ptr_with_offset = new_ptr.add(new_bucket as usize);

                    println!("New bucket for key {} is index {}", entry.key, new_bucket);
                    ptr::write(ptr_with_offset, entry.clone());
                }
            }
        }
    }

    fn shrink(&mut self) {
        todo!()
    }
}

impl Deref for Table {
    type Target = [Entry];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.table.as_ptr(), self.capacity) }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<Entry>();

        if self.capacity != 0 && elem_size != 0 {
            unsafe {
                dealloc(
                    self.table.as_ptr() as *mut u8,
                    Layout::array::<Entry>(self.capacity).unwrap(),
                );
            }
        }
    }
}

unsafe impl Send for Table {}
unsafe impl Sync for Table {}

#[cfg(test)]
mod tests {
    use std::io::Write;

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
        let key1 = RoxString::new("Hello");
        let key2 = RoxString::new("world");
        let key3 = RoxString::new("explosives");
        let key4 = RoxString::new("overwatch");
        let key5 = RoxString::new("anime!?");
        let key6 = RoxString::new("however");
        let value1 = Value::Number(RoxNumber(45.0));
        let value2 = Value::Number(RoxNumber(90.0));
        let value3 = Value::Number(RoxNumber(180.0));
        let value4 = Value::Number(RoxNumber(360.0));
        let value5 = Value::Number(RoxNumber(15.0));
        let value6 = Value::Number(RoxNumber(75.0));

        let entry1 = Entry {
            key: key1.clone(),
            value: value1.clone(),
        };
        let entry2 = Entry {
            key: key2.clone(),
            value: value2.clone(),
        };
        let entry3 = Entry {
            key: key3.clone(),
            value: value3.clone(),
        };
        let entry4 = Entry {
            key: key4.clone(),
            value: value4.clone(),
        };
        let entry5 = Entry {
            key: key5.clone(),
            value: value5.clone(),
        };
        let entry6 = Entry {
            key: key6.clone(),
            value: value6.clone(),
        };

        table.set(&key1, &value1);
        table.set(&key2, &value2);
        table.set(&key3, &value3);
        table.set(&key4, &value4);
        table.set(&key5, &value5);
        table.set(&key6, &value6);

        assert_eq!(table.get(&key1), Some(entry1));
        assert_eq!(table.get(&key2), Some(entry2));
        assert_eq!(table.get(&key3), Some(entry3));
        assert_eq!(table.get(&key4), Some(entry4));
        assert_eq!(table.get(&key5), Some(entry5));
        assert_eq!(table.get(&key6), Some(entry6));

        assert_eq!(table.size(), 6);
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
