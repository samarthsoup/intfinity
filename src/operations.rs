use std::ops::{Add,Sub,Mul,Div};

use crate::intfinity::{SingleInfiniteNumber,DoubleInfiniteNumber};
use crate::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Negate, Unsigned, Zero};

/// Implementation of the `Add` trait for `DoubleInfiniteNumber`.
///
/// This allows for adding two `DoubleInfiniteNumber` values together. The addition follows the rules:
/// - Finite values are added normally, but if the addition results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Adding positive infinity to any value results in positive infinity.
/// - Adding negative infinity to any value results in negative infinity.
/// - Adding positive infinity to negative infinity results in a panic, as it is an indeterminate form.
///
/// # Examples
///
/// ```
/// use intfinity::DoubleInfiniteNumber;
/// let a = DoubleInfiniteNumber::new(5);
/// let b = DoubleInfiniteNumber::PosInfinity;
/// let result = a + b;
/// assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
/// ```
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

/// Implementation of the `Sub` trait for `DoubleInfiniteNumber`.
///
/// This allows for subtracting one `DoubleInfiniteNumber` value from another. The subtraction follows the rules:
/// - Finite values are subtracted normally, but if the subtraction results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Subtracting a finite value from positive infinity or subtracting negative infinity from a finite value results in positive infinity.
/// - Subtracting a finite value from negative infinity or subtracting positive infinity from a finite value results in negative infinity.
/// - Subtracting positive infinity from positive infinity results in a panic, as it is an indeterminate form.
///
/// # Examples
///
/// ```
/// use intfinity::DoubleInfiniteNumber;
/// let a = DoubleInfiniteNumber::new(10);
/// let b = DoubleInfiniteNumber::new(3);
/// let result = a - b;
/// assert_eq!(result, DoubleInfiniteNumber::Finite(7));
/// ```
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

/// Implementation of the `Mul` trait for `DoubleInfiniteNumber`.
///
/// This allows for multiplying two `DoubleInfiniteNumber` values. The multiplication follows the rules:
/// - Finite values are multiplied normally, but if the multiplication results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Multiplying infinity, negative infinity by zero results in a panic as this is considered an indeterminate form.
/// - Multiplying infinity by a positive finite value results in infinity, while multiplying infinity by a negative finite value results in negative infinity.
/// - Multiplying negative infinity by a positive finite value results in negative infinity, while multiplying negative infinity by a negative finite value results in infinity.
///
/// # Examples
///
/// ```
/// use intfinity::DoubleInfiniteNumber;
/// let a = DoubleInfiniteNumber::new(4);
/// let b = DoubleInfiniteNumber::new(5);
/// let result = a * b;
/// assert_eq!(result, DoubleInfiniteNumber::Finite(20));
/// ```
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

/// Implementation of the `Div` trait for `DoubleInfiniteNumber`.
///
/// This allows for dividing one `DoubleInfiniteNumber` value by another. The division follows the rules:
/// - Finite values are divided normally, but if the division results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Division by zero results in a panic.
/// - Dividing infinity by infinity results in a panic, as it is an indeterminate form.
/// - Dividing infinity by a positive finite value results in infinity, while dividing infinity by a negative finite value results in negative infinity.
/// - Dividing negative infinity by a positive finite value results in negative infinity, while dividing negative infinity by a negative finite value results in infinity.
/// - Dividing a finite value by infinty (any sign) results in zero, as implemented for type T.
///
/// # Examples
///
/// ```
/// use intfinity::DoubleInfiniteNumber;
/// let a = DoubleInfiniteNumber::new(20);
/// let b = DoubleInfiniteNumber::new(5);
/// let result = a / b;
/// assert_eq!(result, DoubleInfiniteNumber::Finite(4));
/// ```
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
    /// Compares two `DoubleInfiniteNumber` values in an indeterminate manner.
    ///
    /// - If both values are `Finite`, they are compared using `PartialOrd`.
    /// - If both values are `PosInfinity` or `NegInfinity`, the comparison returns `None`, indicating that they cannot be meaningfully compared.
    /// - If one value is `PosInfinity` and the other is `NegInfinity`, they are ordered accordingly.
    /// - Otherwise, infinities are treated as greater or less than finite values.
    ///
    /// # Example
    /// ```
    /// use intfinity::DoubleInfiniteNumber;
    /// let a: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    /// let b = DoubleInfiniteNumber::NegInfinity;
    ///
    /// match a.indeterminate_cmp(&b) {
    ///     Some(ordering) => println!("Ordering: {:?}", ordering),
    ///     None => println!("Incomparable values"),
    /// }
    /// ```
    pub fn indeterminate_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DoubleInfiniteNumber::Finite(a), DoubleInfiniteNumber::Finite(b)) => a.partial_cmp(b),
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::PosInfinity) => None,
            (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::NegInfinity) => None,
            (DoubleInfiniteNumber::PosInfinity, DoubleInfiniteNumber::NegInfinity) => Some(std::cmp::Ordering::Greater),  
            (DoubleInfiniteNumber::NegInfinity, DoubleInfiniteNumber::PosInfinity) => Some(std::cmp::Ordering::Less),  
            (DoubleInfiniteNumber::PosInfinity, _) => Some(std::cmp::Ordering::Greater),
            (_, DoubleInfiniteNumber::PosInfinity) => Some(std::cmp::Ordering::Less),
            (DoubleInfiniteNumber::NegInfinity, _) => Some(std::cmp::Ordering::Less),
            (_, DoubleInfiniteNumber::NegInfinity) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl<T> Add for SingleInfiniteNumber<T>
where
    T: Copy + Add<Output = T> + PartialOrd + Unsigned + CheckedAdd,
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
                panic!("Division by zero");
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


