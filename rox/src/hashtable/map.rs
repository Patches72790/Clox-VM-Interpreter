use crate::{RoxString, Value};
use std::rc::Rc;

pub trait RoxMap<K = Rc<RoxString>, V = Value> {
    fn get(&self, key: &K) -> Option<&V>;

    fn set(&mut self, key: &K, value: &V) -> bool;

    fn get_and_set(&mut self, key: &K, value: &V) -> bool;

    fn contains(&self, key: K) -> bool;

    fn remove(&mut self, key: K) -> Option<V>;
}
