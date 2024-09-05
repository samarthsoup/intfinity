use core::ops::{Add,Sub,Mul,Div};

use crate::intfinity::{SingleInfiniteNumber,DoubleInfiniteNumber};
use crate::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Negate, Unsigned, Zero};

impl<T> Add for DoubleInfiniteNumber<T>
where
    T: Copy + Add<Output = T> + PartialOrd + Zero + CheckedAdd,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite + finite
            (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::Finite(b)) => {
                a.checked_add(b)
                    .map_or_else(
                        || if a > T::zero() { Self::PosInfinity } else { Self::NegInfinity },
                        DoubleInfiniteNumber::Finite
                    )
            },
            // inf + (-inf)
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::NegInfinity) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::PosInfinity) => {
                panic!("indeterminate form: +inf + (-inf)")
            },
            // inf + x = inf
            (DoubleInfiniteNumber::PosInfinity, _) | (_, DoubleInfiniteNumber::PosInfinity) => Self::PosInfinity,
            // -inf + x = -inf
            (DoubleInfiniteNumber::NegInfinity, _) | (_, DoubleInfiniteNumber::NegInfinity) => Self::NegInfinity,
        }
    }
}

impl<T> Sub for DoubleInfiniteNumber<T>
where
    T: Copy + Sub<Output = T> + PartialOrd + Zero + CheckedSub,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite - finite
            (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::Finite(b)) => {
                a.checked_sub(b)
                    .map_or_else(
                        || if a > T::zero() { Self::PosInfinity } else { Self::NegInfinity },
                        DoubleInfiniteNumber::Finite
                    )
            },
            // inf - inf
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::PosInfinity) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::NegInfinity) => {
                panic!("indeterminate form: inf - inf")
            },
            // inf - x || x - (-inf)
            (DoubleInfiniteNumber::PosInfinity, _) | (_, DoubleInfiniteNumber::NegInfinity) => Self::PosInfinity,
            // -inf - x || x - inf
            (DoubleInfiniteNumber::NegInfinity, _) | (_, DoubleInfiniteNumber::PosInfinity) => Self::NegInfinity,
        }
    }
}

impl<T> Mul for DoubleInfiniteNumber<T>
where
    T: Copy + Mul<Output = T> + PartialOrd + Zero + CheckedMul,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite * finite
            (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::Finite(b)) => {
                a.checked_mul(b)
                    .map_or_else(
                        || {
                            if (a > T::zero() && b > T::zero()) || (a < T::zero() && b < T::zero()) {
                                Self::PosInfinity
                            } else {
                                Self::NegInfinity
                            }
                        },
                        DoubleInfiniteNumber::Finite,
                    )
            },
            /*
                inf * x = {
                    undefined, x = 0;
                    +inf, x > 0;
                    -inf, x < 0
                }
            */
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::Finite(a)) | (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::PosInfinity) => {
                if a.is_zero() {
                    panic!("indefinite form: 0 * inf") 
                } else if a > T::zero() {
                    Self::PosInfinity
                } else {
                    Self::NegInfinity
                }
            },
            /*
                -inf * x = {
                    undefined, x = 0;
                    -inf, x > 0;
                    +inf, x < 0
                }
            */
            (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::Finite(a)) | (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::NegInfinity) => {
                if a.is_zero() {
                    panic!("indefinite form: 0 * -inf")  
                } else if a > T::zero() {
                    Self::NegInfinity
                } else {
                    Self::PosInfinity
                }
            },
            // inf * inf || -inf * (-inf)
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::PosInfinity) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::NegInfinity) => {
                Self::PosInfinity
            },
            // -inf * inf
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::NegInfinity) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::PosInfinity) => {
                Self::NegInfinity
            },
        }
    }
}

impl<T> Div for DoubleInfiniteNumber<T>
where
    T: Copy + Div<Output = T> + PartialOrd + Zero + CheckedDiv + Negate,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite/finite
            (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::Finite(b)) => {
                if b.is_zero() {
                    panic!("division by zero")
                } else {
                    a.checked_div(b)
                        .map_or_else(
                            || if (a > T::zero() && b > T::zero()) || (a < T::zero() && b < T::zero()) {
                                Self::PosInfinity
                            } else {
                                Self::NegInfinity
                            },
                            DoubleInfiniteNumber::Finite
                        )
                }
            },
            // x/inf
            (DoubleInfiniteNumber::Finite(_), DoubleInfiniteNumber::PosInfinity) | (DoubleInfiniteNumber::Finite(_), DoubleInfiniteNumber::NegInfinity) => {
                DoubleInfiniteNumber::Finite(T::zero())
            },
            // inf/x
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::Finite(a)) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::Finite(a)) => {
                if a.is_zero() {
                    panic!("division by zero")
                } else if a > T::zero() {
                    self
                } else {
                    self.negate_double_bounded_infinity()
                }
            },
            // inf/inf
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::PosInfinity) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::NegInfinity) => {
                panic!("indeterminate form: inf / inf")
            },
            // -(inf/inf)
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::NegInfinity) | (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::PosInfinity) => {
                panic!("indeterminate form: inf / -inf")
            },
        }
    }
}

impl<T> DoubleInfiniteNumber<T>
where
    T: PartialEq + PartialOrd,
{
    pub fn indeterminate_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::Finite(b)) => a.partial_cmp(b),
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::PosInfinity) => None,
            (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::NegInfinity) => None,
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::NegInfinity) => Some(core::cmp::Ordering::Greater),  
            (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::PosInfinity) => Some(core::cmp::Ordering::Less),  
            (DoubleInfiniteNumber::PosInfinity, _) => Some(core::cmp::Ordering::Greater),
            (_, DoubleInfiniteNumber::PosInfinity) => Some(core::cmp::Ordering::Less),
            (DoubleInfiniteNumber::NegInfinity, _) => Some(core::cmp::Ordering::Less),
            (_, DoubleInfiniteNumber::NegInfinity) => Some(core::cmp::Ordering::Greater),
        }
    }
}

impl<T> Add for SingleInfiniteNumber<T>
where
    T: Copy + Add<Output = T> + PartialOrd + Unsigned + CheckedAdd + Zero,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Finite(a), Self::Finite(b)) => {
                match a.checked_add(b) {
                    Some(sum) => Self::Finite(sum),
                    None => Self::Infinity, 
                }
            }
            (Self::Infinity, _) | (_, Self::Infinity) => Self::Infinity,
        }
    }
}

impl<T> Sub for SingleInfiniteNumber<T>
where
    T: Copy + Sub<Output = T> + PartialOrd + Unsigned + CheckedSub + Zero,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Finite(a), Self::Finite(b)) => {
                match a.checked_sub(b) {
                    Some(diff) => Self::Finite(diff),
                    None => Self::Finite(T::zero()), 
                }
            }
            (Self::Infinity, _) => Self::Infinity,
            (_, Self::Infinity) => Self::Finite(T::zero()), 
        }
    }
}

impl<T> Mul for SingleInfiniteNumber<T>
where
    T: Copy + Mul<Output = T> + PartialOrd + Unsigned + CheckedMul + Zero,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Finite(a), Self::Finite(b)) => {
                match a.checked_mul(b) {
                    Some(prod) => Self::Finite(prod),
                    None => Self::Infinity, 
                }
            },
            (Self::Infinity, Self::Finite(a)) | (Self::Finite(a), Self::Infinity) => {
                if a.is_zero() {
                    panic!("indeterminate form: 0 * inf");
                } else {
                    Self::Infinity
                }
            },
            (Self::Infinity, Self::Infinity) => Self::Infinity,
        }
    }
}

impl<T> Div for SingleInfiniteNumber<T>
where
    T: Copy + Div<Output = T> + PartialOrd + Unsigned + CheckedDiv + Zero,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Finite(_), Self::Finite(b)) if b.is_zero() => {
                panic!("division by zero");
            }
            (Self::Finite(a), Self::Finite(b)) => {
                match a.checked_div(b) {
                    Some(quot) => Self::Finite(quot),
                    None => Self::Infinity, 
                }
            }
            (Self::Infinity, Self::Finite(_)) => Self::Infinity, 
            (Self::Finite(_), Self::Infinity) => Self::Finite(T::zero()), 
            (Self::Infinity, Self::Infinity) => panic!("indeterminate form: inf / inf"), 
        }
    }
}

impl<T> SingleInfiniteNumber<T>
where
    T: PartialEq + PartialOrd + Unsigned,
{
    pub fn indeterminate_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (SingleInfiniteNumber::Finite(a), SingleInfiniteNumber::Finite(b)) => a.partial_cmp(b),
            (SingleInfiniteNumber::Infinity, SingleInfiniteNumber::Infinity) => None,
            (SingleInfiniteNumber::Infinity, _) => Some(core::cmp::Ordering::Greater),
            (_, SingleInfiniteNumber::Infinity) => Some(core::cmp::Ordering::Less)
        }
    }
}


