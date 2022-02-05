extern crate precedence_macro;
use precedence_macro::make_precedence;

#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(0)]
pub struct PrecNone;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(1)]
pub struct PrecAssign;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(2)]
pub struct PrecOr;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(3)]
pub struct PrecAnd;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(4)]
pub struct PrecEquality;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(5)]
pub struct PrecComparison;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(6)]
pub struct PrecTerm;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(7)]
pub struct PrecFactor;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(8)]
pub struct PrecUnary;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(9)]
pub struct PrecCall;
#[derive(PartialOrd, PartialEq, Debug)]
#[make_precedence(10)]
pub struct PrecPrimary;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Precedence {
    PrecNone,
    PrecAssign,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary,
}

impl Precedence {
    pub fn get_next(&self) -> &Self {
        match self {
            Precedence::PrecNone => &Precedence::PrecAssign,
            Precedence::PrecAssign => &Precedence::PrecOr,
            Precedence::PrecOr => &Precedence::PrecAnd,
            Precedence::PrecAnd => &Precedence::PrecEquality,
            Precedence::PrecEquality => &Precedence::PrecComparison,
            Precedence::PrecComparison => &Precedence::PrecTerm,
            Precedence::PrecTerm => &Precedence::PrecFactor,
            Precedence::PrecFactor => &Precedence::PrecUnary,
            Precedence::PrecUnary => &Precedence::PrecCall,
            Precedence::PrecCall => &Precedence::PrecPrimary,
            Precedence::PrecPrimary => panic!("Error, no precedence higher than PrePrimary"),
        }
    }
}

impl std::ops::Deref for Precedence {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            Precedence::PrecNone => &*PrecNone,
            Precedence::PrecAssign => &*PrecAssign,
            Precedence::PrecOr => &*PrecOr,
            Precedence::PrecAnd => &*PrecAnd,
            Precedence::PrecEquality => &*PrecEquality,
            Precedence::PrecComparison => &*PrecComparison,
            Precedence::PrecTerm => &*PrecTerm,
            Precedence::PrecFactor => &*PrecFactor,
            Precedence::PrecUnary => &*PrecUnary,
            Precedence::PrecCall => &*PrecCall,
            Precedence::PrecPrimary => &*PrecPrimary,
        }
    }
}

impl std::fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Precedence::PrecNone => write!(f, "PrecNone"),
            Precedence::PrecAssign => write!(f, "PrecAssign"),
            Precedence::PrecOr => write!(f, "PrecOr"),
            Precedence::PrecAnd => write!(f, "PrecAnd"),
            Precedence::PrecEquality => write!(f, "PrecEquality"),
            Precedence::PrecComparison => write!(f, "PrecComparison"),
            Precedence::PrecTerm => write!(f, "PrecTerm"),
            Precedence::PrecFactor => write!(f, "PrecFactor"),
            Precedence::PrecUnary => write!(f, "PrecUnary"),
            Precedence::PrecCall => write!(f, "PrecCall"),
            Precedence::PrecPrimary => write!(f, "PrecPrimary"),
        }
    }
}
