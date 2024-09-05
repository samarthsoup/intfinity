use crate::traits::{Zero, Negate, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Unsigned};
use crate::{DoubleInfiniteNumber, SingleInfiniteNumber};

impl Zero for i8 {
    fn zero() -> Self {
        0
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

impl From<i8> for DoubleInfiniteNumber<i8> {
    fn from(value: i8) -> Self {
        DoubleInfiniteNumber::Finite(value)
    }
}

impl Into<i8> for DoubleInfiniteNumber<i8> {
    fn into(self) -> i8 {
        match self {
            DoubleInfiniteNumber::Finite(val) => val,
            DoubleInfiniteNumber::PosInfinity => i8::MAX, 
            DoubleInfiniteNumber::NegInfinity => i8::MIN,
        }
    }
}

impl Zero for i16 {
    fn zero() -> Self {
        0
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

impl From<i16> for DoubleInfiniteNumber<i16> {
    fn from(value: i16) -> Self {
        DoubleInfiniteNumber::Finite(value)
    }
}

impl Into<i16> for DoubleInfiniteNumber<i16> {
    fn into(self) -> i16 {
        match self {
            DoubleInfiniteNumber::Finite(val) => val,
            DoubleInfiniteNumber::PosInfinity => i16::MAX, 
            DoubleInfiniteNumber::NegInfinity => i16::MIN,
        }
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
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

impl From<i32> for DoubleInfiniteNumber<i32> {
    fn from(value: i32) -> Self {
        DoubleInfiniteNumber::Finite(value)
    }
}

impl Into<i32> for DoubleInfiniteNumber<i32> {
    fn into(self) -> i32 {
        match self {
            DoubleInfiniteNumber::Finite(val) => val,
            DoubleInfiniteNumber::PosInfinity => i32::MAX, 
            DoubleInfiniteNumber::NegInfinity => i32::MIN,
        }
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
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

impl From<i64> for DoubleInfiniteNumber<i64> {
    fn from(value: i64) -> Self {
        DoubleInfiniteNumber::Finite(value)
    }
}

impl Into<i64> for DoubleInfiniteNumber<i64> {
    fn into(self) -> i64 {
        match self {
            DoubleInfiniteNumber::Finite(val) => val,
            DoubleInfiniteNumber::PosInfinity => i64::MAX, 
            DoubleInfiniteNumber::NegInfinity => i64::MIN,
        }
    }
}

impl Zero for i128 {
    fn zero() -> Self {
        0
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

impl From<i128> for DoubleInfiniteNumber<i128> {
    fn from(value: i128) -> Self {
        DoubleInfiniteNumber::Finite(value)
    }
}

impl Into<i128> for DoubleInfiniteNumber<i128> {
    fn into(self) -> i128 {
        match self {
            DoubleInfiniteNumber::Finite(val) => val,
            DoubleInfiniteNumber::PosInfinity => i128::MAX, 
            DoubleInfiniteNumber::NegInfinity => i128::MIN,
        }
    }
}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}

impl Zero for u8 {
    fn zero() -> Self {
        0
    }
}

impl CheckedAdd for u8 {
    fn checked_add(self, other: u8) -> Option<u8> {
        self.checked_add(other)
    }
}

impl CheckedSub for u8 {
    fn checked_sub(self, other: u8) -> Option<u8> {
        self.checked_sub(other)
    }
}

impl CheckedMul for u8 {
    fn checked_mul(self, other: u8) -> Option<u8> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for u8 {
    fn checked_div(self, other: u8) -> Option<u8> {
        self.checked_div(other)
    }
}

impl From<u8> for SingleInfiniteNumber<u8> {
    fn from(value: u8) -> Self {
        SingleInfiniteNumber::Finite(value)
    }
}

impl Into<u8> for SingleInfiniteNumber<u8> {
    fn into(self) -> u8 {
        match self {
            SingleInfiniteNumber::Finite(val) => val,
            SingleInfiniteNumber::Infinity => u8::MAX, 
        }
    }
}

impl Zero for u16 {
    fn zero() -> Self {
        0
    }
}

impl CheckedAdd for u16 {
    fn checked_add(self, other: u16) -> Option<u16> {
        self.checked_add(other)
    }
}

impl CheckedSub for u16 {
    fn checked_sub(self, other: u16) -> Option<u16> {
        self.checked_sub(other)
    }
}

impl CheckedMul for u16 {
    fn checked_mul(self, other: u16) -> Option<u16> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for u16 {
    fn checked_div(self, other: u16) -> Option<u16> {
        self.checked_div(other)
    }
}

impl From<u16> for SingleInfiniteNumber<u16> {
    fn from(value: u16) -> Self {
        SingleInfiniteNumber::Finite(value)
    }
}

impl Into<u16> for SingleInfiniteNumber<u16> {
    fn into(self) -> u16 {
        match self {
            SingleInfiniteNumber::Finite(val) => val,
            SingleInfiniteNumber::Infinity => u16::MAX, 
        }
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl CheckedAdd for u32 {
    fn checked_add(self, other: u32) -> Option<u32> {
        self.checked_add(other)
    }
}

impl CheckedSub for u32 {
    fn checked_sub(self, other: u32) -> Option<u32> {
        self.checked_sub(other)
    }
}

impl CheckedMul for u32 {
    fn checked_mul(self, other: u32) -> Option<u32> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for u32 {
    fn checked_div(self, other: u32) -> Option<u32> {
        self.checked_div(other)
    }
}

impl From<u32> for SingleInfiniteNumber<u32> {
    fn from(value: u32) -> Self {
        SingleInfiniteNumber::Finite(value)
    }
}

impl Into<u32> for SingleInfiniteNumber<u32> {
    fn into(self) -> u32 {
        match self {
            SingleInfiniteNumber::Finite(val) => val,
            SingleInfiniteNumber::Infinity => u32::MAX, 
        }
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}

impl CheckedAdd for u64 {
    fn checked_add(self, other: u64) -> Option<u64> {
        self.checked_add(other)
    }
}

impl CheckedSub for u64 {
    fn checked_sub(self, other: u64) -> Option<u64> {
        self.checked_sub(other)
    }
}

impl CheckedMul for u64 {
    fn checked_mul(self, other: u64) -> Option<u64> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for u64 {
    fn checked_div(self, other: u64) -> Option<u64> {
        self.checked_div(other)
    }
}

impl From<u64> for SingleInfiniteNumber<u64> {
    fn from(value: u64) -> Self {
        SingleInfiniteNumber::Finite(value)
    }
}

impl Into<u64> for SingleInfiniteNumber<u64> {
    fn into(self) -> u64 {
        match self {
            SingleInfiniteNumber::Finite(val) => val,
            SingleInfiniteNumber::Infinity => u64::MAX, 
        }
    }
}

impl Zero for u128 {
    fn zero() -> Self {
        0
    }
}

impl CheckedAdd for u128 {
    fn checked_add(self, other: u128) -> Option<u128> {
        self.checked_add(other)
    }
}

impl CheckedSub for u128 {
    fn checked_sub(self, other: u128) -> Option<u128> {
        self.checked_sub(other)
    }
}

impl CheckedMul for u128 {
    fn checked_mul(self, other: u128) -> Option<u128> {
        self.checked_mul(other)
    }
}

impl CheckedDiv for u128 {
    fn checked_div(self, other: u128) -> Option<u128> {
        self.checked_div(other)
    }
}

impl From<u128> for SingleInfiniteNumber<u128> {
    fn from(value: u128) -> Self {
        SingleInfiniteNumber::Finite(value)
    }
}

impl Into<u128> for SingleInfiniteNumber<u128> {
    fn into(self) -> u128 {
        match self {
            SingleInfiniteNumber::Finite(val) => val,
            SingleInfiniteNumber::Infinity => u128::MAX, 
        }
    }
}