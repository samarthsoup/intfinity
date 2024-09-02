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

pub trait Negate {
    fn negate(self) -> Self;
}

pub trait CheckedAdd: Sized {
    fn checked_add(self, other: Self) -> Option<Self>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(self, other: Self) -> Option<Self>;
}

pub trait CheckedMul: Sized {
    fn checked_mul(self, other: Self) -> Option<Self>;
}

pub trait CheckedDiv: Sized {
    fn checked_div(self, other: Self) -> Option<Self>;
}

impl<T> Intfinity<T>
where
    T: Copy + Add<Output = T> + PartialOrd + Zero + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + Negate
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

impl<T> Intfinity<T>
where
    T: Copy + Negate,
{
    pub fn negate_intfinity(self) -> Self {
        match self {
            Intfinity::Finite(value) => Intfinity::Finite(value.negate()), 
            Intfinity::PosInfinity => Intfinity::NegInfinity,  
            Intfinity::NegInfinity => Intfinity::PosInfinity,  
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
    #[should_panic(expected = "indeterminate form: +inf + (-inf)")]
    fn test_addition_positive_and_negative_infinity() {
        let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
        let neg_inf = Intfinity::NegInfinity;

        let _result = pos_inf + neg_inf;
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
    #[should_panic(expected = "indeterminate form: inf - inf")]
    fn test_subtraction_inf_minus_inf_should_panic() {
        let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
        let neg_inf: Intfinity<i32> = Intfinity::NegInfinity;

        let _result = pos_inf - pos_inf;
        let _result = neg_inf - neg_inf;
    }

    #[test]
    fn test_subtraction_with_overflow() {
        let a = Intfinity::new(i32::MAX);
        let b = Intfinity::new(-1);
        let result = a - b;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_multiplication_finite_values() {
        let a = Intfinity::new(4);
        let b = Intfinity::new(5);
        let result = a * b;
        assert_eq!(result, Intfinity::Finite(20));
    }

    #[test]
    fn test_multiplication_with_overflow() {
        let a = Intfinity::new(i32::MAX);
        let b = Intfinity::new(2);
        let result = a * b;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    #[should_panic(expected = "indefinite form: 0 * inf")]
    fn test_multiplication_by_zero_and_infinity() {
        let a = Intfinity::new(0);
        let b = Intfinity::PosInfinity;
        let _result = a * b; 
    }

    #[test]
    #[should_panic(expected = "indefinite form: 0 * -inf")]
    fn test_multiplication_by_zero_and_negative_infinity() {
        let a = Intfinity::new(0);
        let b = Intfinity::NegInfinity;
        let _result = a * b;  
    }

    #[test]
    fn test_multiplication_with_infinity() {
        let a = Intfinity::new(4);
        let result = a * Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::PosInfinity);

        let a = Intfinity::new(-4);
        let result = a * Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::NegInfinity);

        let a = Intfinity::new(4);
        let result = a * Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::NegInfinity);

        let a = Intfinity::new(-4);
        let result = a * Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_multiplication_infinity_by_infinity() {
        let result: Intfinity<i32> = Intfinity::PosInfinity * Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::PosInfinity);

        let result: Intfinity<i32> = Intfinity::NegInfinity * Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::PosInfinity);

        let result: Intfinity<i32> = Intfinity::PosInfinity * Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::NegInfinity);

        let result: Intfinity<i32> = Intfinity::NegInfinity * Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_multiplication_with_negative_numbers() {
        let a = Intfinity::new(-3);
        let b = Intfinity::new(6);
        let result = a * b;
        assert_eq!(result, Intfinity::Finite(-18));

        let result = a * Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::NegInfinity);

        let result = a * Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_multiplication_with_positive_numbers() {
        let a = Intfinity::new(3);
        let b = Intfinity::new(6);
        let result = a * b;
        assert_eq!(result, Intfinity::Finite(18));

        let result = a * Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::PosInfinity);

        let result = a * Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_division_finite_values() {
        let a = Intfinity::new(10);
        let b = Intfinity::new(2);
        let result = a / b;
        assert_eq!(result, Intfinity::Finite(5));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_division_by_zero() {
        let a = Intfinity::new(10);
        let b = Intfinity::new(0);
        let _result = a / b;  
    }

    #[test]
    fn test_division_by_infinity() {
        let a = Intfinity::new(10);
        let result = a / Intfinity::PosInfinity;
        assert_eq!(result, Intfinity::Finite(0)); 

        let result = a / Intfinity::NegInfinity;
        assert_eq!(result, Intfinity::Finite(0)); 
    }

    #[test]
    fn test_infinity_divided_by_finite_value() {
        let a = Intfinity::PosInfinity;
        let b = Intfinity::new(2);
        let result = a / b;
        assert_eq!(result, Intfinity::PosInfinity);

        let a = Intfinity::NegInfinity;
        let result = a / b;
        assert_eq!(result, Intfinity::NegInfinity);

        let b = Intfinity::new(-2);
        let result = a / b;
        assert_eq!(result, Intfinity::PosInfinity);

        let a = Intfinity::PosInfinity;
        let result = a / b;
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    #[should_panic(expected = "indeterminate form: inf / inf")]
    fn test_infinity_divided_by_infinity_should_panic() {
        let a: Intfinity<i32> = Intfinity::PosInfinity;
        let b: Intfinity<i32> = Intfinity::PosInfinity;
        let _result = a / b; 

        let a: Intfinity<i32> = Intfinity::NegInfinity;
        let b: Intfinity<i32> = Intfinity::NegInfinity;
        let _result = a / b;  
    }

    #[test]
    #[should_panic(expected = "indeterminate form: inf / -inf")]
    fn test_pos_infinity_divided_by_neg_infinity_should_panic() {
        let a: Intfinity<i32> = Intfinity::PosInfinity;
        let b: Intfinity<i32> = Intfinity::NegInfinity;

        let _result = a / b;  
        let _result = b / a;  
    }

    #[test]
    fn test_finite_divided_by_finite() {
        let a = Intfinity::new(10);
        let b = Intfinity::new(2);
        let result = a / b;
        assert_eq!(result, Intfinity::Finite(5));

        let a = Intfinity::new(-10);
        let b = Intfinity::new(2);
        let result = a / b;
        assert_eq!(result, Intfinity::Finite(-5));

        let a = Intfinity::new(10);
        let b = Intfinity::new(-2);
        let result = a / b;
        assert_eq!(result, Intfinity::Finite(-5));

        let a = Intfinity::new(-10);
        let b = Intfinity::new(-2);
        let result = a / b;
        assert_eq!(result, Intfinity::Finite(5));
    }

    #[test]
    fn test_negate_finite_positive() {
        let a = Intfinity::Finite(10);
        let result = a.negate_intfinity();
        assert_eq!(result, Intfinity::Finite(-10));
    }

    #[test]
    fn test_negate_finite_negative() {
        let a = Intfinity::Finite(-10);
        let result = a.negate_intfinity();
        assert_eq!(result, Intfinity::Finite(10));
    }

    #[test]
    fn test_negate_pos_infinity() {
        let a = Intfinity::PosInfinity::<i32>;
        let result = a.negate_intfinity();
        assert_eq!(result, Intfinity::NegInfinity);
    }

    #[test]
    fn test_negate_neg_infinity() {
        let a = Intfinity::NegInfinity::<i32>;
        let result = a.negate_intfinity();
        assert_eq!(result, Intfinity::PosInfinity);
    }

    #[test]
    fn test_negate_zero() {
        let a = Intfinity::Finite(0);
        let result = a.negate_intfinity();
        assert_eq!(result, Intfinity::Finite(0));
    }

    #[test]
    fn test_negate_large_value() {
        let a = Intfinity::Finite(100000);
        let result = a.negate_intfinity();
        assert_eq!(result, Intfinity::Finite(-100000));
    }
}
