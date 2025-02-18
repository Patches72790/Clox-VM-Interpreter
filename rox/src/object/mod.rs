mod roxstring;

pub use crate::object::roxstring::RoxString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoxObject {
    pub object_type: ObjectType,
}

impl RoxObject {
    pub fn new(object_type: ObjectType) -> RoxObject {
        RoxObject { object_type }
    }
}

impl std::fmt::Display for RoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.object_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
