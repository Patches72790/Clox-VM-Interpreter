use crate::RoxString;
use crate::Value;
use std::collections::HashMap;

pub struct Table {
    inner_table: HashMap<RoxString, Value>,
}
