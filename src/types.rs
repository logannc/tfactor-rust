use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Mul, Div};
use std::cmp::{Eq, PartialEq, PartialOrd, Ord};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Display for Literal {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Int(i) => write!(fmt, "{}", i),
            Self::Float(f) => write!(fmt, "{}", f),
            Self::String(s) => write!(fmt, "{}", s),
            Self::Boolean(b) => write!(fmt, "{}", b),
        }
    }
}

impl Add<Literal> for Literal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a + b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a + b),
            (Self::String(a), Self::String(b)) => Self::String(a + &b),
            (a, b) => panic!("Type Error: No implementation for Add for non-numerals or mixed types: {:?}, {:?}", a, b)
        }
    }
}

impl Mul<Literal> for Literal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a * b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a * b),
            (a, b) => panic!("Type Error: No implementation for Mul for non-numerals or mixed types: {:?}, {:?}", a, b)
        }
    }
}

impl Div<Literal> for Literal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a / b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a / b),
            (a, b) => panic!("Type Error: No implementation for Div for non-numerals or mixed types: {:?}, {:?}", a, b)
        }
    }
}