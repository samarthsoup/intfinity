use crate::traits::{Zero, Negate, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use crate::Intfinity;

impl Zero for i8 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl Negate for i8 {
    fn negate(self) -> Self {
        -self
    }
}

impl CheckedAdd for i8 {
    fn checked_add(self, other: i8) -> Option<i8> {
        self.checked_add(other)
    }
}

impl CheckedSub for i8 {
    fn checked_sub(self, other: i8) -> Option<i8> {
        self.checked_sub(other)
    }
}

impl CheckedMul for i8 {
    fn checked_mul(self, other: i8) -> Option<i8> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for i8 {
    fn checked_div(self, other: i8) -> Option<i8> {
        self.checked_div(other)
    }
}

impl From<i8> for Intfinity<i8> {
    fn from(value: i8) -> Self {
        Intfinity::Finite(value)
    }
}

impl Into<i8> for Intfinity<i8> {
    fn into(self) -> i8 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i8::MAX, 
            Intfinity::NegInfinity => i8::MIN,
        }
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl Negate for i16 {
    fn negate(self) -> Self {
        -self
    }
}

impl CheckedAdd for i16 {
    fn checked_add(self, other: i16) -> Option<i16> {
        self.checked_add(other)
    }
}

impl CheckedSub for i16 {
    fn checked_sub(self, other: i16) -> Option<i16> {
        self.checked_sub(other)
    }
}

impl CheckedMul for i16 {
    fn checked_mul(self, other: i16) -> Option<i16> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for i16 {
    fn checked_div(self, other: i16) -> Option<i16> {
        self.checked_div(other)
    }
}

impl From<i16> for Intfinity<i16> {
    fn from(value: i16) -> Self {
        Intfinity::Finite(value)
    }
}

impl Into<i16> for Intfinity<i16> {
    fn into(self) -> i16 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i16::MAX, 
            Intfinity::NegInfinity => i16::MIN,
        }
    }
}

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

impl Zero for i64 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl Negate for i64 {
    fn negate(self) -> Self {
        -self
    }
}

impl CheckedAdd for i64 {
    fn checked_add(self, other: i64) -> Option<i64> {
        self.checked_add(other)
    }
}

impl CheckedSub for i64 {
    fn checked_sub(self, other: i64) -> Option<i64> {
        self.checked_sub(other)
    }
}

impl CheckedMul for i64 {
    fn checked_mul(self, other: i64) -> Option<i64> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for i64 {
    fn checked_div(self, other: i64) -> Option<i64> {
        self.checked_div(other)
    }
}

impl From<i64> for Intfinity<i64> {
    fn from(value: i64) -> Self {
        Intfinity::Finite(value)
    }
}

impl Into<i64> for Intfinity<i64> {
    fn into(self) -> i64 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i64::MAX, 
            Intfinity::NegInfinity => i64::MIN,
        }
    }
}

impl Zero for i128 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl Negate for i128 {
    fn negate(self) -> Self {
        -self
    }
}

impl CheckedAdd for i128 {
    fn checked_add(self, other: i128) -> Option<i128> {
        self.checked_add(other)
    }
}

impl CheckedSub for i128 {
    fn checked_sub(self, other: i128) -> Option<i128> {
        self.checked_sub(other)
    }
}

impl CheckedMul for i128 {
    fn checked_mul(self, other: i128) -> Option<i128> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for i128 {
    fn checked_div(self, other: i128) -> Option<i128> {
        self.checked_div(other)
    }
}

impl From<i128> for Intfinity<i128> {
    fn from(value: i128) -> Self {
        Intfinity::Finite(value)
    }
}

impl Into<i128> for Intfinity<i128> {
    fn into(self) -> i128 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i128::MAX, 
            Intfinity::NegInfinity => i128::MIN,
        }
    }
}