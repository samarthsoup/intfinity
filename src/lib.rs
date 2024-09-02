//hi
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intfinity<T> {
    Finite(T),
    PosInfinity,
    NegInfinity,
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait CheckedAdd: Sized {
    fn checked_add(self, other: Self) -> Option<Self>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(self, other: Self) -> Option<Self>;
}

impl<T> Intfinity<T>
where
    T: Copy + Add<Output = T> + PartialOrd + Zero + CheckedAdd + CheckedSub
{
    // constructor using new
    pub fn new(value: T) -> Self {
        Intfinity::Finite(value)
    }

    // checks if value is zero
    pub fn is_zero(&self) -> bool {
        match self {
            Intfinity::Finite(val) => val.is_zero(),
            _ => false,
        }
    }
}

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
            // (-inf) - x || x - inf
            (Intfinity::NegInfinity, _) | (_, Intfinity::PosInfinity) => Self::NegInfinity,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_finite_values() {
        let a = Intfinity::new(10);
        let b = Intfinity::new(20);
        let result = a + b;
        assert_eq!(result, Intfinity::Finite(30));
    }

    #[test]
    fn test_addition_with_overflow() {
        let a = Intfinity::new(i32::MAX);
        let b = Intfinity::new(1);
        let result = a + b;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_addition_with_negative_overflow() {
        let a = Intfinity::new(i32::MIN);
        let b = Intfinity::new(-1);
        let result = a + b;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_addition_positive_and_negative_infinity() {
        let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
        let neg_inf = Intfinity::NegInfinity;

        let result = std::panic::catch_unwind(|| pos_inf + neg_inf);
        assert!(result.is_err());  // panic because indeterminate form
    }

    #[test]
    fn test_addition_with_positive_infinity() {
        let a = Intfinity::new(10);
        let result = a + Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_addition_with_negative_infinity() {
        let a = Intfinity::new(-10);
        let result = a + Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_subtraction_finite_values() {
        let a = Intfinity::new(20);
        let b = Intfinity::new(10);
        let result = a - b;
        assert_eq!(result, Intfinity::Finite(10));
    }

    #[test]
    fn test_subtraction_with_underflow() {
        let a = Intfinity::new(i32::MIN);
        let b = Intfinity::new(1);
        let result = a - b;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_subtraction_positive_infinity() {
        let a = Intfinity::PosInfinity;
        let b = Intfinity::new(10);
        let result = a - b;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_subtraction_negative_infinity() {
        let a = Intfinity::NegInfinity;
        let b = Intfinity::new(10);
        let result = a - b;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_subtraction_inf_minus_neg_inf() {
        let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
        let neg_inf = Intfinity::NegInfinity;
        let result = pos_inf - neg_inf;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_subtraction_neg_inf_minus_pos_inf() {
        let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
        let neg_inf = Intfinity::NegInfinity;
        let result = neg_inf - pos_inf;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_subtraction_inf_minus_inf_should_panic() {
        let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
        let neg_inf: Intfinity<i32> = Intfinity::NegInfinity;

        let result = std::panic::catch_unwind(|| pos_inf - pos_inf);
        assert!(result.is_err());  // Should panic due to indeterminate form

        let result = std::panic::catch_unwind(|| neg_inf - neg_inf);
        assert!(result.is_err());  // Should also panic due to indeterminate form
    }

    #[test]
    fn test_subtraction_with_overflow() {
        let a = Intfinity::new(i32::MAX);
        let b = Intfinity::new(-1);
        let result = a - b;
        assert_eq!(result, Intfinity::PosInfinity);
    }
}
