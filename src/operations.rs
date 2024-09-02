use std::ops::{Add,Sub,Mul,Div};

use crate::intfinity::Intfinity;
use crate::traits::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Zero, Negate};

impl<T> Add for Intfinity<T>
where
    T: Copy + Add<Output = T> + PartialOrd + Zero + CheckedAdd,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite + finite
            (Intfinity::Finite(a), Intfinity::Finite(b)) => {
                a.checked_add(b)
                    .map_or_else(
                        || if a > T::zero() { Self::PosInfinity } else { Self::NegInfinity },
                        Intfinity::Finite
                    )
            },
            // inf + (-inf)
            (Intfinity::PosInfinity, Intfinity::NegInfinity) | (Intfinity::NegInfinity, Intfinity::PosInfinity) => {
                panic!("indeterminate form: +inf + (-inf)")
            },
            // inf + x = inf
            (Intfinity::PosInfinity, _) | (_, Intfinity::PosInfinity) => Self::PosInfinity,
            // -inf + x = -inf
            (Intfinity::NegInfinity, _) | (_, Intfinity::NegInfinity) => Self::NegInfinity,
        }
    }
}

impl<T> Sub for Intfinity<T>
where
    T: Copy + Sub<Output = T> + PartialOrd + Zero + CheckedSub,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite - finite
            (Intfinity::Finite(a), Intfinity::Finite(b)) => {
                a.checked_sub(b)
                    .map_or_else(
                        || if a > T::zero() { Self::PosInfinity } else { Self::NegInfinity },
                        Intfinity::Finite
                    )
            },
            // inf - inf
            (Intfinity::PosInfinity, Intfinity::PosInfinity) | (Intfinity::NegInfinity, Intfinity::NegInfinity) => {
                panic!("indeterminate form: inf - inf")
            },
            // inf - x || x - (-inf)
            (Intfinity::PosInfinity, _) | (_, Intfinity::NegInfinity) => Self::PosInfinity,
            // -inf - x || x - inf
            (Intfinity::NegInfinity, _) | (_, Intfinity::PosInfinity) => Self::NegInfinity,
        }
    }
}

impl<T> Mul for Intfinity<T>
where
    T: Copy + Mul<Output = T> + PartialOrd + Zero + CheckedMul,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite * finite
            (Intfinity::Finite(a), Intfinity::Finite(b)) => {
                a.checked_mul(b)
                    .map_or_else(
                        || {
                            if (a > T::zero() && b > T::zero()) || (a < T::zero() && b < T::zero()) {
                                Self::PosInfinity
                            } else {
                                Self::NegInfinity
                            }
                        },
                        Intfinity::Finite,
                    )
            },
            /*
                inf * x = {
                    undefined, x = 0;
                    +inf, x > 0;
                    -inf, x < 0
                }
            */
            (Intfinity::PosInfinity, Intfinity::Finite(a)) | (Intfinity::Finite(a), Intfinity::PosInfinity) => {
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
            (Intfinity::NegInfinity, Intfinity::Finite(a)) | (Intfinity::Finite(a), Intfinity::NegInfinity) => {
                if a.is_zero() {
                    panic!("indefinite form: 0 * -inf")  
                } else if a > T::zero() {
                    Self::NegInfinity
                } else {
                    Self::PosInfinity
                }
            },
            // inf * inf || -inf * (-inf)
            (Intfinity::PosInfinity, Intfinity::PosInfinity) | (Intfinity::NegInfinity, Intfinity::NegInfinity) => {
                Self::PosInfinity
            },
            // -inf * inf
            (Intfinity::PosInfinity, Intfinity::NegInfinity) | (Intfinity::NegInfinity, Intfinity::PosInfinity) => {
                Self::NegInfinity
            },
        }
    }
}

impl<T> Div for Intfinity<T>
where
    T: Copy + Div<Output = T> + PartialOrd + Zero + CheckedDiv + Negate,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            // finite/finite
            (Intfinity::Finite(a), Intfinity::Finite(b)) => {
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
                            Intfinity::Finite
                        )
                }
            },
            // x/inf
            (Intfinity::Finite(_), Intfinity::PosInfinity) | (Intfinity::Finite(_), Intfinity::NegInfinity) => {
                Intfinity::Finite(T::zero())
            },
            // inf/x
            (Intfinity::PosInfinity, Intfinity::Finite(a)) | (Intfinity::NegInfinity, Intfinity::Finite(a)) => {
                if a.is_zero() {
                    panic!("division by zero")
                } else if a > T::zero() {
                    self
                } else {
                    self.negate_intfinity()
                }
            },
            // inf/inf
            (Intfinity::PosInfinity, Intfinity::PosInfinity) | (Intfinity::NegInfinity, Intfinity::NegInfinity) => {
                panic!("indeterminate form: inf / inf")
            },
            // -(inf/inf)
            (Intfinity::PosInfinity, Intfinity::NegInfinity) | (Intfinity::NegInfinity, Intfinity::PosInfinity) => {
                panic!("indeterminate form: inf / -inf")
            },
        }
    }
}