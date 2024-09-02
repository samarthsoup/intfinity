use crate::traits::{Zero, Negate, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use crate::Intfinity;

impl Zero for i32 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl Negate for i32 {
    fn negate(self) -> Self {
        -self
    }
}

impl CheckedAdd for i32 {
    fn checked_add(self, other: i32) -> Option<i32> {
        self.checked_add(other)
    }
}

impl CheckedSub for i32 {
    fn checked_sub(self, other: i32) -> Option<i32> {
        self.checked_sub(other)
    }
}

impl CheckedMul for i32 {
    fn checked_mul(self, other: i32) -> Option<i32> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for i32 {
    fn checked_div(self, other: i32) -> Option<i32> {
        self.checked_div(other)
    }
}

impl From<i32> for Intfinity<i32> {
    fn from(value: i32) -> Self {
        Intfinity::Finite(value)
    }
}

impl Into<i32> for Intfinity<i32> {
    fn into(self) -> i32 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i32::MAX, 
            Intfinity::NegInfinity => i32::MIN,
        }
    }
}