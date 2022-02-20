mod roxstring;

use crate::object::roxstring::RoxString;

#[derive(Debug, Clone)]
pub enum ObjectType {
    ObjString(RoxString),
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::ObjString(string) => write!(f, "str = {}", string),
        }
    }
}
