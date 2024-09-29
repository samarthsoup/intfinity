use crate::traits::{Zero, Negate, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Unsigned};
use crate::{DoubleInfiniteNumber, SingleInfiniteNumber};

macro_rules! impl_double_infinity_numeric_primitives_traits {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }

        impl Negate for $t {
            fn negate(self) -> Self {
                -self
            }
        }

        impl CheckedAdd for $t {
            fn checked_add(self, other: $t) -> Option<$t> {
                self.checked_add(other)
            }
        }

        impl CheckedSub for $t {
            fn checked_sub(self, other: $t) -> Option<$t> {
                self.checked_sub(other)
            }
        }

        impl CheckedMul for $t {
            fn checked_mul(self, other: $t) -> Option<$t> {
                self.checked_mul(other)
            }
        }

        impl CheckedDiv for $t {
            fn checked_div(self, other: $t) -> Option<$t> {
                self.checked_div(other)
            }
        }

        impl From<$t> for DoubleInfiniteNumber<$t> {
            fn from(value: $t) -> Self {
                DoubleInfiniteNumber::Finite(value)
            }
        }

        impl Into<$t> for DoubleInfiniteNumber<$t> {
            fn into(self) -> $t {
                match self {
                    DoubleInfiniteNumber::Finite(val) => val,
                    DoubleInfiniteNumber::PosInfinity => <$t>::MAX,
                    DoubleInfiniteNumber::NegInfinity => <$t>::MIN,
                }
            }
        }
    };
}

macro_rules! impl_single_infinity_numeric_primitives_traits {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }

        impl CheckedAdd for $t {
            fn checked_add(self, other: $t) -> Option<$t> {
                self.checked_add(other)
            }
        }

        impl CheckedSub for $t {
            fn checked_sub(self, other: $t) -> Option<$t> {
                self.checked_sub(other)
            }
        }

        impl CheckedMul for $t {
            fn checked_mul(self, other: $t) -> Option<$t> {
                self.checked_mul(other)
            }
        }

        impl CheckedDiv for $t {
            fn checked_div(self, other: $t) -> Option<$t> {
                self.checked_div(other)
            }
        }

        impl From<$t> for SingleInfiniteNumber<$t> {
            fn from(value: $t) -> Self {
                SingleInfiniteNumber::Finite(value)
            }
        }

        impl Into<$t> for SingleInfiniteNumber<$t> {
            fn into(self) -> $t {
                match self {
                    SingleInfiniteNumber::Finite(val) => val,
                    SingleInfiniteNumber::Infinity => <$t>::MAX,
                }
            }
        }
    };
}

impl_double_infinity_numeric_primitives_traits!(i8);
impl_double_infinity_numeric_primitives_traits!(i16);
impl_double_infinity_numeric_primitives_traits!(i32);
impl_double_infinity_numeric_primitives_traits!(i64);
impl_double_infinity_numeric_primitives_traits!(i128);

impl Unsigned for u8{}
impl Unsigned for u16{}
impl Unsigned for u32{}
impl Unsigned for u64{}
impl Unsigned for u128{}
impl_single_infinity_numeric_primitives_traits!(u8);
impl_single_infinity_numeric_primitives_traits!(u16);
impl_single_infinity_numeric_primitives_traits!(u32);
impl_single_infinity_numeric_primitives_traits!(u64);
impl_single_infinity_numeric_primitives_traits!(u128);