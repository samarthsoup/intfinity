use crate::traits::{Zero, Negate, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use crate::Intfinity;

/// Implementation of `Zero` trait for `i8`, providing the zero value and checking if a value is zero.
impl Zero for i8 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `Negate` trait for `i8`, providing the negation of a value.
impl Negate for i8 {
    fn negate(self) -> Self {
        -self
    }
}

/// Implementation of `CheckedAdd` trait for `i8`, providing checked addition.
impl CheckedAdd for i8 {
    fn checked_add(self, other: i8) -> Option<i8> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `i8`, providing checked subtraction.
impl CheckedSub for i8 {
    fn checked_sub(self, other: i8) -> Option<i8> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `i8`, providing checked multiplication.
impl CheckedMul for i8 {
    fn checked_mul(self, other: i8) -> Option<i8> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `i8`, providing checked division.
impl CheckedDiv for i8 {
    fn checked_div(self, other: i8) -> Option<i8> {
        self.checked_div(other)
    }
}

/// Implementation of `From<i8>` for `Intfinity<i8>`, allowing conversion from `i8` to `Intfinity`.
impl From<i8> for Intfinity<i8> {
    fn from(value: i8) -> Self {
        Intfinity::Finite(value)
    }
}

/// Implementation of `Into<i8>` for `Intfinity<i8>`, allowing conversion from `Intfinity` to `i8`.
impl Into<i8> for Intfinity<i8> {
    fn into(self) -> i8 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i8::MAX, 
            Intfinity::NegInfinity => i8::MIN,
        }
    }
}

/// Implementation of `Zero` trait for `i16`, providing the zero value and checking if a value is zero.
impl Zero for i16 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `Negate` trait for `i16`, providing the negation of a value.
impl Negate for i16 {
    fn negate(self) -> Self {
        -self
    }
}

/// Implementation of `CheckedAdd` trait for `i16`, providing checked addition.
impl CheckedAdd for i16 {
    fn checked_add(self, other: i16) -> Option<i16> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `i16`, providing checked subtraction.
impl CheckedSub for i16 {
    fn checked_sub(self, other: i16) -> Option<i16> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `i16`, providing checked multiplication.
impl CheckedMul for i16 {
    fn checked_mul(self, other: i16) -> Option<i16> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `i16`, providing checked division.
impl CheckedDiv for i16 {
    fn checked_div(self, other: i16) -> Option<i16> {
        self.checked_div(other)
    }
}

/// Implementation of `From<i16>` for `Intfinity<i16>`, allowing conversion from `i16` to `Intfinity`.
impl From<i16> for Intfinity<i16> {
    fn from(value: i16) -> Self {
        Intfinity::Finite(value)
    }
}

/// Implementation of `Into<i16>` for `Intfinity<i16>`, allowing conversion from `Intfinity` to `i16`.
impl Into<i16> for Intfinity<i16> {
    fn into(self) -> i16 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i16::MAX, 
            Intfinity::NegInfinity => i16::MIN,
        }
    }
}

/// Implementation of `Zero` trait for `i32`, providing the zero value and checking if a value is zero.
impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `Negate` trait for `i32`, providing the negation of a value.
impl Negate for i32 {
    fn negate(self) -> Self {
        -self
    }
}

/// Implementation of `CheckedAdd` trait for `i32`, providing checked addition.
impl CheckedAdd for i32 {
    fn checked_add(self, other: i32) -> Option<i32> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `i32`, providing checked subtraction.
impl CheckedSub for i32 {
    fn checked_sub(self, other: i32) -> Option<i32> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `i32`, providing checked multiplication.
impl CheckedMul for i32 {
    fn checked_mul(self, other: i32) -> Option<i32> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `i32`, providing checked division.
impl CheckedDiv for i32 {
    fn checked_div(self, other: i32) -> Option<i32> {
        self.checked_div(other)
    }
}

/// Implementation of `From<i32>` for `Intfinity<i32>`, allowing conversion from `i32` to `Intfinity`.
impl From<i32> for Intfinity<i32> {
    fn from(value: i32) -> Self {
        Intfinity::Finite(value)
    }
}

/// Implementation of `Into<i32>` for `Intfinity<i32>`, allowing conversion from `Intfinity` to `i32`.
impl Into<i32> for Intfinity<i32> {
    fn into(self) -> i32 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i32::MAX, 
            Intfinity::NegInfinity => i32::MIN,
        }
    }
}

/// Implementation of `Zero` trait for `i64`, providing the zero value and checking if a value is zero.
impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `Negate` trait for `i64`, providing the negation of a value.
impl Negate for i64 {
    fn negate(self) -> Self {
        -self
    }
}

/// Implementation of `CheckedAdd` trait for `i64`, providing checked addition.
impl CheckedAdd for i64 {
    fn checked_add(self, other: i64) -> Option<i64> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `i64`, providing checked subtraction.
impl CheckedSub for i64 {
    fn checked_sub(self, other: i64) -> Option<i64> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `i64`, providing checked multiplication.
impl CheckedMul for i64 {
    fn checked_mul(self, other: i64) -> Option<i64> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `i64`, providing checked division.
impl CheckedDiv for i64 {
    fn checked_div(self, other: i64) -> Option<i64> {
        self.checked_div(other)
    }
}

/// Implementation of `From<i64>` for `Intfinity<i64>`, allowing conversion from `i64` to `Intfinity`.
impl From<i64> for Intfinity<i64> {
    fn from(value: i64) -> Self {
        Intfinity::Finite(value)
    }
}

/// Implementation of `Into<i64>` for `Intfinity<i64>`, allowing conversion from `Intfinity` to `i64`.
impl Into<i64> for Intfinity<i64> {
    fn into(self) -> i64 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i64::MAX, 
            Intfinity::NegInfinity => i64::MIN,
        }
    }
}

/// Implementation of `Zero` trait for `i128`, providing the zero value and checking if a value is zero.
impl Zero for i128 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `Negate` trait for `i128`, providing the negation of a value.
impl Negate for i128 {
    fn negate(self) -> Self {
        -self
    }
}

/// Implementation of `CheckedAdd` trait for `i128`, providing checked addition.
impl CheckedAdd for i128 {
    fn checked_add(self, other: i128) -> Option<i128> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `i128`, providing checked subtraction.
impl CheckedSub for i128 {
    fn checked_sub(self, other: i128) -> Option<i128> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `i128`, providing checked multiplication.
impl CheckedMul for i128 {
    fn checked_mul(self, other: i128) -> Option<i128> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `i128`, providing checked division.
impl CheckedDiv for i128 {
    fn checked_div(self, other: i128) -> Option<i128> {
        self.checked_div(other)
    }
}

/// Implementation of `From<i128>` for `Intfinity<i128>`, allowing conversion from `i128` to `Intfinity`.
impl From<i128> for Intfinity<i128> {
    fn from(value: i128) -> Self {
        Intfinity::Finite(value)
    }
}

/// Implementation of `Into<i128>` for `Intfinity<i128>`, allowing conversion from `Intfinity` to `i128`.
impl Into<i128> for Intfinity<i128> {
    fn into(self) -> i128 {
        match self {
            Intfinity::Finite(val) => val,
            Intfinity::PosInfinity => i128::MAX, 
            Intfinity::NegInfinity => i128::MIN,
        }
    }
}