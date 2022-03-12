extern crate precedence_macro;
use precedence_macro::make_precedence;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
pub enum NewPrecedence {
    PrecNone = 0,
    PrecAssign = 1,
    PrecOr = 2,
    PrecAnd = 3,
    PrecEquality = 4,
    PrecComparison = 5,
    PrecTerm = 6,
    PrecFactor = 7,
    PrecUnary = 8,
    PrecCall = 9,
    PrecPrimary = 10,
}

impl Into<u8> for NewPrecedence {
    fn into(self) -> u8 {
        match self {
            NewPrecedence::PrecNone => 0,
            NewPrecedence::PrecAssign => 1,
            NewPrecedence::PrecOr => 2,
            NewPrecedence::PrecAnd => 3,
            NewPrecedence::PrecEquality => 4,
            NewPrecedence::PrecComparison => 5,
            NewPrecedence::PrecTerm => 6,
            NewPrecedence::PrecFactor => 7,
            NewPrecedence::PrecUnary => 8,
            NewPrecedence::PrecCall => 9,
            NewPrecedence::PrecPrimary => 10,
        }
    }
}

impl From<u8> for NewPrecedence {
    fn from(n: u8) -> Self {
        match n {
            0 => NewPrecedence::PrecNone,
            1 => NewPrecedence::PrecAssign,
            2 => NewPrecedence::PrecOr,
            3 => NewPrecedence::PrecAnd,
            4 => NewPrecedence::PrecEquality,
            5 => NewPrecedence::PrecComparison,
            6 => NewPrecedence::PrecTerm,
            7 => NewPrecedence::PrecFactor,
            8 => NewPrecedence::PrecUnary,
            9 => NewPrecedence::PrecCall,
            10 => NewPrecedence::PrecPrimary,
            unknown => panic!("Cannot yield u8 from unknown Precedence Value {}", unknown),
        }
    }
}

impl std::ops::Add<u8> for NewPrecedence {
    type Output = NewPrecedence;

    fn add(self, rhs: u8) -> Self::Output {
        let n: u8 = self.into();

        NewPrecedence::from(n + rhs)
    }
}

impl std::fmt::Display for NewPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewPrecedence::PrecNone => write!(f, "PrecNone"),
            NewPrecedence::PrecAssign => write!(f, "PrecAssign"),
            NewPrecedence::PrecOr => write!(f, "PrecOr"),
            NewPrecedence::PrecAnd => write!(f, "PrecAnd"),
            NewPrecedence::PrecEquality => write!(f, "PrecEquality"),
            NewPrecedence::PrecComparison => write!(f, "PrecComparison"),
            NewPrecedence::PrecTerm => write!(f, "PrecTerm"),
            NewPrecedence::PrecFactor => write!(f, "PrecFactor"),
            NewPrecedence::PrecUnary => write!(f, "PrecUnary"),
            NewPrecedence::PrecCall => write!(f, "PrecCall"),
            NewPrecedence::PrecPrimary => write!(f, "PrecPrimary"),
        }
    }
}

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
