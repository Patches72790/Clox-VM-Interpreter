use std::ops::Deref;

impl Deref for Assign {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &0u8
    }
}

pub struct Assign;
pub struct Or;

pub enum Precedence {
    PrecAssign(Assign),
    PrecOr(Or),
}
