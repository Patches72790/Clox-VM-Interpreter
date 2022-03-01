use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoxString(Rc<String>);

impl RoxString {
    pub fn new(string: Rc<String>) -> RoxString {
        RoxString(string)
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes().clone()
    }

    pub fn raw_parts(&mut self) -> (*const u8, usize, usize) {
        (self.0.as_ptr(), self.0.len(), self.0.capacity())
    }
}

impl Deref for RoxString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl std::ops::Add for RoxString {
    type Output = RoxString;

    fn add(self, rhs: Self) -> Self::Output {
        let str1 = self.0.as_str();

        let mut new_string = String::from(str1);
        new_string.push_str(rhs.0.as_str());

        RoxString::new(Rc::new(new_string))
    }
}

impl std::fmt::Display for RoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
