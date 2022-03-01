use crate::hashtable::entry::Entry;
use crate::{RoxString, Value};
use std::rc::Rc;

pub trait RoxMap<K = Rc<RoxString>, V = Value, E = Entry> {
    fn get(&self, key: K) -> Option<&E>;

    fn set(&mut self, key: K, value: &V);

    fn contains(&self, key: K) -> bool;

    fn remove(&mut self, key: K) -> Option<E>;
}
