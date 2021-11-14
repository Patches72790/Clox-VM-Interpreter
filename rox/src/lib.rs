pub mod chunk {
    pub enum OpCode {
        OpReturn,
    }

    #[derive(Debug)]
    pub struct Chunk {
        count: i32,
        capacity: i32,
        code: Option<Vec<i8>>,
    }

    fn grow_capacity(capacity: i32) -> i32 {
        if capacity < 8 {
            8
        } else {
            capacity * 2
        }
    }

    impl Chunk {
        pub fn new() -> Chunk {
            Chunk {
                count: 0,
                capacity: 0,
                code: None,
            }
        }

        pub fn write_chunk(&mut self, byte: i8) {
            if self.capacity < self.count + 1 {
                let old_capacity = self.capacity;
                self.capacity = grow_capacity(old_capacity);
            }

            self.code = match &self.code {
                Some(v) => {
                    let mut new_result = v.clone();
                    new_result.push(byte);
                    Some(new_result)
                }
                None => Some(vec![byte]),
            };
            self.count = self.count + 1;
        }
    }
}
