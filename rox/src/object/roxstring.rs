use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoxString(String);

impl RoxString {
    pub fn new(string: &str) -> RoxString {
        RoxString(string.to_string())
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Deref for RoxString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for RoxString {
    type Output = RoxString;

    fn add(self, rhs: Self) -> Self::Output {
        RoxString::new(&(self.0 + &rhs.0))
    }
}

impl std::fmt::Display for RoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
