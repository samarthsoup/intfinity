use crate::traits::{Zero, Negate, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use crate::{DoubleBoundedInfinity, SingleBoundedInfinity};

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

/// Implementation of `From<i8>` for `DoubleBoundedInfinity<i8>`, allowing conversion from `i8` to `DoubleBoundedInfinity`.
impl From<i8> for DoubleBoundedInfinity<i8> {
    fn from(value: i8) -> Self {
        DoubleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<i8>` for `DoubleBoundedInfinity<i8>`, allowing conversion from `DoubleBoundedInfinity` to `i8`.
impl Into<i8> for DoubleBoundedInfinity<i8> {
    fn into(self) -> i8 {
        match self {
            DoubleBoundedInfinity::Finite(val) => val,
            DoubleBoundedInfinity::PosInfinity => i8::MAX, 
            DoubleBoundedInfinity::NegInfinity => i8::MIN,
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

/// Implementation of `From<i16>` for `DoubleBoundedInfinity<i16>`, allowing conversion from `i16` to `DoubleBoundedInfinity`.
impl From<i16> for DoubleBoundedInfinity<i16> {
    fn from(value: i16) -> Self {
        DoubleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<i16>` for `DoubleBoundedInfinity<i16>`, allowing conversion from `DoubleBoundedInfinity` to `i16`.
impl Into<i16> for DoubleBoundedInfinity<i16> {
    fn into(self) -> i16 {
        match self {
            DoubleBoundedInfinity::Finite(val) => val,
            DoubleBoundedInfinity::PosInfinity => i16::MAX, 
            DoubleBoundedInfinity::NegInfinity => i16::MIN,
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

/// Implementation of `From<i32>` for `DoubleBoundedInfinity<i32>`, allowing conversion from `i32` to `DoubleBoundedInfinity`.
impl From<i32> for DoubleBoundedInfinity<i32> {
    fn from(value: i32) -> Self {
        DoubleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<i32>` for `DoubleBoundedInfinity<i32>`, allowing conversion from `DoubleBoundedInfinity` to `i32`.
impl Into<i32> for DoubleBoundedInfinity<i32> {
    fn into(self) -> i32 {
        match self {
            DoubleBoundedInfinity::Finite(val) => val,
            DoubleBoundedInfinity::PosInfinity => i32::MAX, 
            DoubleBoundedInfinity::NegInfinity => i32::MIN,
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

/// Implementation of `From<i64>` for `DoubleBoundedInfinity<i64>`, allowing conversion from `i64` to `DoubleBoundedInfinity`.
impl From<i64> for DoubleBoundedInfinity<i64> {
    fn from(value: i64) -> Self {
        DoubleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<i64>` for `DoubleBoundedInfinity<i64>`, allowing conversion from `DoubleBoundedInfinity` to `i64`.
impl Into<i64> for DoubleBoundedInfinity<i64> {
    fn into(self) -> i64 {
        match self {
            DoubleBoundedInfinity::Finite(val) => val,
            DoubleBoundedInfinity::PosInfinity => i64::MAX, 
            DoubleBoundedInfinity::NegInfinity => i64::MIN,
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

/// Implementation of `From<i128>` for `DoubleBoundedInfinity<i128>`, allowing conversion from `i128` to `DoubleBoundedInfinity`.
impl From<i128> for DoubleBoundedInfinity<i128> {
    fn from(value: i128) -> Self {
        DoubleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<i128>` for `DoubleBoundedInfinity<i128>`, allowing conversion from `DoubleBoundedInfinity` to `i128`.
impl Into<i128> for DoubleBoundedInfinity<i128> {
    fn into(self) -> i128 {
        match self {
            DoubleBoundedInfinity::Finite(val) => val,
            DoubleBoundedInfinity::PosInfinity => i128::MAX, 
            DoubleBoundedInfinity::NegInfinity => i128::MIN,
        }
    }
}

/// Implementation of `Zero` trait for `u8`, providing the zero value and checking if a value is zero.
impl Zero for u8 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `CheckedAdd` trait for `i128`, providing checked addition.
impl CheckedAdd for u8 {
    fn checked_add(self, other: u8) -> Option<u8> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `u8`, providing checked subtraction.
impl CheckedSub for u8 {
    fn checked_sub(self, other: u8) -> Option<u8> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `u8`, providing checked multiplication.
impl CheckedMul for u8 {
    fn checked_mul(self, other: u8) -> Option<u8> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `u8`, providing checked division.
impl CheckedDiv for u8 {
    fn checked_div(self, other: u8) -> Option<u8> {
        self.checked_div(other)
    }
}

/// Implementation of `From<u8>` for `SingleBoundedInfinity<u8>`, allowing conversion from `u8` to `SingleBoundedInfinity`.
impl From<u8> for SingleBoundedInfinity<u8> {
    fn from(value: u8) -> Self {
        SingleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<u8>` for `SingleBoundedInfinity<u8>`, allowing conversion from `SingleBoundedInfinity` to `u8`.
impl Into<u8> for SingleBoundedInfinity<u8> {
    fn into(self) -> u8 {
        match self {
            SingleBoundedInfinity::Finite(val) => val,
            SingleBoundedInfinity::Infinity => u8::MAX, 
        }
    }
}

/// Implementation of `Zero` trait for `u16`, providing the zero value and checking if a value is zero.
impl Zero for u16 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `CheckedAdd` trait for `u16`, providing checked addition.
impl CheckedAdd for u16 {
    fn checked_add(self, other: u16) -> Option<u16> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `u16`, providing checked subtraction.
impl CheckedSub for u16 {
    fn checked_sub(self, other: u16) -> Option<u16> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `u16`, providing checked multiplication.
impl CheckedMul for u16 {
    fn checked_mul(self, other: u16) -> Option<u16> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `u16`, providing checked division.
impl CheckedDiv for u16 {
    fn checked_div(self, other: u16) -> Option<u16> {
        self.checked_div(other)
    }
}

/// Implementation of `From<u16>` for `SingleBoundedInfinity<u16>`, allowing conversion from `u16` to `SingleBoundedInfinity`.
impl From<u16> for SingleBoundedInfinity<u16> {
    fn from(value: u16) -> Self {
        SingleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<u16>` for `SingleBoundedInfinity<u16>`, allowing conversion from `SingleBoundedInfinity` to `u16`.
impl Into<u16> for SingleBoundedInfinity<u16> {
    fn into(self) -> u16 {
        match self {
            SingleBoundedInfinity::Finite(val) => val,
            SingleBoundedInfinity::Infinity => u16::MAX, 
        }
    }
}

/// Implementation of `Zero` trait for `u32`, providing the zero value and checking if a value is zero.
impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `CheckedAdd` trait for `u32`, providing checked addition.
impl CheckedAdd for u32 {
    fn checked_add(self, other: u32) -> Option<u32> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `u32`, providing checked subtraction.
impl CheckedSub for u32 {
    fn checked_sub(self, other: u32) -> Option<u32> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `u32`, providing checked multiplication.
impl CheckedMul for u32 {
    fn checked_mul(self, other: u32) -> Option<u32> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `u32`, providing checked division.
impl CheckedDiv for u32 {
    fn checked_div(self, other: u32) -> Option<u32> {
        self.checked_div(other)
    }
}

/// Implementation of `From<u32>` for `SingleBoundedInfinity<u32>`, allowing conversion from `u32` to `SingleBoundedInfinity`.
impl From<u32> for SingleBoundedInfinity<u32> {
    fn from(value: u32) -> Self {
        SingleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<u32>` for `SingleBoundedInfinity<u32>`, allowing conversion from `SingleBoundedInfinity` to `u32`.
impl Into<u32> for SingleBoundedInfinity<u32> {
    fn into(self) -> u32 {
        match self {
            SingleBoundedInfinity::Finite(val) => val,
            SingleBoundedInfinity::Infinity => u32::MAX, 
        }
    }
}

/// Implementation of `Zero` trait for `u64`, providing the zero value and checking if a value is zero.
impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `CheckedAdd` trait for `u64`, providing checked addition.
impl CheckedAdd for u64 {
    fn checked_add(self, other: u64) -> Option<u64> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `u64`, providing checked subtraction.
impl CheckedSub for u64 {
    fn checked_sub(self, other: u64) -> Option<u64> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `u64`, providing checked multiplication.
impl CheckedMul for u64 {
    fn checked_mul(self, other: u64) -> Option<u64> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `u64`, providing checked division.
impl CheckedDiv for u64 {
    fn checked_div(self, other: u64) -> Option<u64> {
        self.checked_div(other)
    }
}

/// Implementation of `From<u64>` for `SingleBoundedInfinity<u64>`, allowing conversion from `u64` to `SingleBoundedInfinity`.
impl From<u64> for SingleBoundedInfinity<u64> {
    fn from(value: u64) -> Self {
        SingleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<u64>` for `SingleBoundedInfinity<u64>`, allowing conversion from `SingleBoundedInfinity` to `u64`.
impl Into<u64> for SingleBoundedInfinity<u64> {
    fn into(self) -> u64 {
        match self {
            SingleBoundedInfinity::Finite(val) => val,
            SingleBoundedInfinity::Infinity => u64::MAX, 
        }
    }
}

/// Implementation of `Zero` trait for `u128`, providing the zero value and checking if a value is zero.
impl Zero for u128 {
    fn zero() -> Self {
        0
    }
}

/// Implementation of `CheckedAdd` trait for `u128`, providing checked addition.
impl CheckedAdd for u128 {
    fn checked_add(self, other: u128) -> Option<u128> {
        self.checked_add(other)
    }
}

/// Implementation of `CheckedSub` trait for `u128`, providing checked subtraction.
impl CheckedSub for u128 {
    fn checked_sub(self, other: u128) -> Option<u128> {
        self.checked_sub(other)
    }
}

/// Implementation of `CheckedMul` trait for `u128`, providing checked multiplication.
impl CheckedMul for u128 {
    fn checked_mul(self, other: u128) -> Option<u128> {
        self.checked_mul(other)
    }
}

/// Implementation of `CheckedDiv` trait for `u128`, providing checked division.
impl CheckedDiv for u128 {
    fn checked_div(self, other: u128) -> Option<u128> {
        self.checked_div(other)
    }
}

/// Implementation of `From<u128>` for `SingleBoundedInfinity<u128>`, allowing conversion from `u128` to `SingleBoundedInfinity`.
impl From<u128> for SingleBoundedInfinity<u128> {
    fn from(value: u128) -> Self {
        SingleBoundedInfinity::Finite(value)
    }
}

/// Implementation of `Into<u128>` for `SingleBoundedInfinity<u128>`, allowing conversion from `SingleBoundedInfinity` to `u128`.
impl Into<u128> for SingleBoundedInfinity<u128> {
    fn into(self) -> u128 {
        match self {
            SingleBoundedInfinity::Finite(val) => val,
            SingleBoundedInfinity::Infinity => u128::MAX, 
        }
    }
}