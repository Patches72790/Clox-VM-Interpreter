use std::rc::Rc;

struct Scanner {
    source: Rc<String>,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: Rc::new(source.to_string()),
            line: 1,
        }
    }

    pub fn scan_tokens(&self) {
        for (i, ch) in self.source.chars().enumerate() {}
    }

    fn scan_token(&self) {}
}
