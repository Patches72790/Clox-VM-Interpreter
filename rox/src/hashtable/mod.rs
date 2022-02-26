use crate::value::Value;
use crate::RoxString;
use std::alloc::{alloc, dealloc, realloc, Layout};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

const INITIAL_TABLE_CAPACITY: usize = 8;
static LOAD_FACTOR: f32 = 0.75;

pub trait RoxMap<K = RoxString, V = Value> {
    fn get(&self, key: &K) -> Option<V>;

    fn set(&mut self, key: &K, value: &V);

    fn contains(&self, key: &K) -> bool;

    fn remove(&mut self, key: &K) -> Option<V>;
}

impl RoxMap for Table {
    fn get(&self, key: &RoxString) -> Option<Value> {
        let capacity = self.capacity;
        let hashed_key = self.hash_key(&key);
        let bucket = hashed_key % capacity as u64;

        let value = unsafe { ptr::read(self.table.as_ptr().add(bucket.try_into().unwrap())) };

        Some(value)
    }

    fn set(&mut self, key: &RoxString, value: &Value) {
        if self.length as f32 >= self.load_factor() {
            self.grow();
        }

        let capacity = self.capacity;
        let hashed_key = self.hash_key(&key);
        let bucket = hashed_key % capacity as u64;
        println!("Bucket for key {key} is {bucket} with capacity {capacity}");

        // TODO! need to implement linear probing for insertion to deal
        // with duplicate hashed keys
        unsafe {
            ptr::write(
                self.table.as_ptr().add(bucket.try_into().unwrap()),
                value.clone(),
            );
        }

        self.length += 1;
    }

    fn contains(&self, key: &RoxString) -> bool {
        let capacity = self.capacity;
        let hashed_key = self.hash_key(&key);
        let bucket = hashed_key % capacity as u64;

        let value = unsafe { ptr::read(self.table.as_ptr().add(bucket.try_into().unwrap())) };

        value == value
    }

    fn remove(&mut self, key: &RoxString) -> Option<Value> {
        todo!()
    }
}

pub struct Table {
    table: NonNull<Value>,
    capacity: usize,
    length: usize,
    _marker: PhantomData<Value>,
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

    fn hash_key(&self, key: &RoxString) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(key.clone().as_bytes());
        hasher.finish()
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
                Layout::array::<Value>(INITIAL_TABLE_CAPACITY)
                    .expect("Error initializing layout for table"),
            )
        } else {
            let new_capacity = 2 * self.capacity;
            let new_layout = Layout::array::<Value>(new_capacity)
                .expect("Error increasing size of layout for table");
            (new_capacity, new_layout)
        };

        // this allocates memory for the new layout/capacity
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<Value>(self.capacity).unwrap();
            let old_ptr = self.table.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // get the pointer for the newly allocated memory here
        self.table = match NonNull::new(new_ptr as *mut Value) {
            Some(p) => p,
            None => std::alloc::handle_alloc_error(new_layout),
        };

        self.capacity = new_capacity;
    }

    fn shrink(&mut self) {
        todo!()
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<Value>();

        if self.capacity != 0 && elem_size != 0 {
            unsafe {
                dealloc(
                    self.table.as_ptr() as *mut u8,
                    Layout::array::<Value>(self.capacity).unwrap(),
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
    }

    #[test]
    fn test_basic_table_get_and_set() {
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

        assert_eq!(table.get(&key), Some(value));
        assert_eq!(table.get(&key2), Some(value2));
        assert_eq!(table.get(&key3), Some(value3));
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

        assert!(table.contains(&key));
        assert!(table.contains(&key2));
        assert!(table.contains(&key3));
    }
}
