use crate::traits::{Zero, Negate, Unsigned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoubleInfiniteNumber<T> {
    NegInfinity,
    Finite(T),
    PosInfinity,
}

impl<T> DoubleInfiniteNumber<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        DoubleInfiniteNumber::Finite(value)
    }
}

impl<T> DoubleInfiniteNumber<T>
where
    T: Zero + PartialEq,
{
    pub fn is_zero(&self) -> bool {
        match self {
            DoubleInfiniteNumber::Finite(val) => val.is_zero(),
            _ => false,
        }
    }
}

impl<T> DoubleInfiniteNumber<T>
where
    T: Copy + Negate,
{
    pub fn negate_double_bounded_infinity(self) -> Self {
        match self {
            DoubleInfiniteNumber::Finite(value) => DoubleInfiniteNumber::Finite(value.negate()), 
            DoubleInfiniteNumber::PosInfinity => DoubleInfiniteNumber::NegInfinity,  
            DoubleInfiniteNumber::NegInfinity => DoubleInfiniteNumber::PosInfinity,  
        }
    }
}

impl<T> std::fmt::Display for DoubleInfiniteNumber<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoubleInfiniteNumber::Finite(val) => write!(f, "{}", val),
            DoubleInfiniteNumber::PosInfinity => write!(f, "+infinity"),
            DoubleInfiniteNumber::NegInfinity => write!(f, "-infinity"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SingleInfiniteNumber<T: Unsigned> {
    Finite(T),
    Infinity,
}

impl<T> SingleInfiniteNumber<T>
where
    T: Copy + Unsigned,
{
    pub fn new(value: T) -> Self {
        SingleInfiniteNumber::Finite(value)
    }
}

impl<T> std::fmt::Display for SingleInfiniteNumber<T>
where
    T: std::fmt::Display + Unsigned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleInfiniteNumber::Finite(val) => write!(f, "{}", val),
            SingleInfiniteNumber::Infinity => write!(f, "+infinity"),
        }
    }
}
