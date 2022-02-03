extern crate precedence_macro;

use precedence_macro::make_precedence;
use std::cmp::PartialOrd;
use std::ops::Deref;

#[derive(PartialOrd, PartialEq)]
#[make_precedence(0)]
pub struct Assign;

#[derive(PartialOrd, PartialEq)]
#[make_precedence(1)]
pub struct Or;

pub enum Precedence {
    PrecAssign(Assign),
    PrecOr(Or),
}
