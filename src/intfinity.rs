use crate::traits::{Zero, Negate};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intfinity<T> {
    Finite(T),
    PosInfinity,
    NegInfinity,
}

impl<T> Intfinity<T>
where
    T: Copy,
{
    // constructor using new
    pub fn new(value: T) -> Self {
        Intfinity::Finite(value)
    }
}

impl<T> Intfinity<T>
where
    T: Zero,
{
    // Checks if value is zero
    pub fn is_zero(&self) -> bool {
        match self {
            Intfinity::Finite(val) => val.is_zero(),
            _ => false,
        }
    }
}

impl<T> Intfinity<T>
where
    T: Copy + Negate,
{
    pub fn negate_intfinity(self) -> Self {
        match self {
            Intfinity::Finite(value) => Intfinity::Finite(value.negate()), 
            Intfinity::PosInfinity => Intfinity::NegInfinity,  
            Intfinity::NegInfinity => Intfinity::PosInfinity,  
        }
    }
}