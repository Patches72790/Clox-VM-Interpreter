#[derive(Debug)]
pub struct Values {
    pub count: usize,
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub enum Value {
    Number(f32),
}

impl Values {
    pub fn new() -> Values {
        Values {
            count: 0,
            values: vec![],
        }
    }

    /**
     * Writes a value to the values array and returns the index at which it
     * was added for use in the chunk instruction block.
     */ 
    pub fn write_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.count += 1;
        self.count - 1
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num.to_string()),
        }
    }
}
