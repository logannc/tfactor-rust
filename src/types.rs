use std::fmt::Debug;
use std::ops::{Add, Mul, Div};

#[derive(Debug)]
pub(crate) enum Literal {
    Int(i64)
}

impl Add<Literal> for Literal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a + b)
        }
    }
}

impl Mul<Literal> for Literal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a * b)
        }
    }
}

impl Div<Literal> for Literal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a / b)
        }
    }
}