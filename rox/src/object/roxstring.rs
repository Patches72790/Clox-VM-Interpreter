#[derive(Clone, Debug)]
pub struct RoxString(String);

impl RoxString {
    pub fn new(string: &str) -> RoxString {
        RoxString(string.to_string())
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Display for RoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
