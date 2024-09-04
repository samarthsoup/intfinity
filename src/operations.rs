use std::ops::{Add,Sub,Mul,Div};

use crate::intfinity::Intfinity;
use crate::traits::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Zero, Negate};

/// Implementation of the `Add` trait for `Intfinity`.
///
/// This allows for adding two `Intfinity` values together. The addition follows the rules:
/// - Finite values are added normally, but if the addition results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Adding positive infinity to any value results in positive infinity.
/// - Adding negative infinity to any value results in negative infinity.
/// - Adding positive infinity to negative infinity results in a panic, as it is an indeterminate form.
///
/// # Examples
///
/// ```
/// use intfinity::Intfinity;
/// let a = Intfinity::new(5);
/// let b = Intfinity::PosInfinity;
/// let result = a + b;
/// assert_eq!(result, Intfinity::PosInfinity);
/// ```
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

/// Implementation of the `Sub` trait for `Intfinity`.
///
/// This allows for subtracting one `Intfinity` value from another. The subtraction follows the rules:
/// - Finite values are subtracted normally, but if the subtraction results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Subtracting a finite value from positive infinity or subtracting negative infinity from a finite value results in positive infinity.
/// - Subtracting a finite value from negative infinity or subtracting positive infinity from a finite value results in negative infinity.
/// - Subtracting positive infinity from positive infinity results in a panic, as it is an indeterminate form.
///
/// # Examples
///
/// ```
/// use intfinity::Intfinity;
/// let a = Intfinity::new(10);
/// let b = Intfinity::new(3);
/// let result = a - b;
/// assert_eq!(result, Intfinity::Finite(7));
/// ```
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

/// Implementation of the `Mul` trait for `Intfinity`.
///
/// This allows for multiplying two `Intfinity` values. The multiplication follows the rules:
/// - Finite values are multiplied normally, but if the multiplication results in overflow, underflow; it maps to infinity, negative infinity respectively.
/// - Multiplying infinity, negative infinity by zero results in a panic as this is considered an indeterminate form.
/// - Multiplying infinity by a positive finite value results in infinity, while multiplying infinity by a negative finite value results in negative infinity.
/// - Multiplying negative infinity by a positive finite value results in negative infinity, while multiplying negative infinity by a negative finite value results in infinity.
///
/// # Examples
///
/// ```
/// use intfinity::Intfinity;
/// let a = Intfinity::new(4);
/// let b = Intfinity::new(5);
/// let result = a * b;
/// assert_eq!(result, Intfinity::Finite(20));
/// ```
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

/// Implementation of the `Div` trait for `Intfinity`.
///
/// This allows for dividing one `Intfinity` value by another. The division follows the rules:
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
/// use intfinity::Intfinity;
/// let a = Intfinity::new(20);
/// let b = Intfinity::new(5);
/// let result = a / b;
/// assert_eq!(result, Intfinity::Finite(4));
/// ```
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

impl<T> PartialEq for Intfinity<T>
where
    T: PartialEq,
{
    /// Compares two `Intfinity` values for equality.
    /// 
    /// - If both values are `Finite`, their inner values are compared using `PartialEq`.
    /// - If both values are `PosInfinity`, they are considered equal.
    /// - If both values are `NegInfinity`, they are considered equal.
    /// - Otherwise, the values are not equal.
    ///
    /// # Example
    /// ```
    /// use intfinity::Intfinity;
    /// let a = Intfinity::new(5);
    /// let b = Intfinity::new(5);
    /// let c = Intfinity::PosInfinity;
    ///
    /// assert!(a == b);
    /// assert!(a != c);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Intfinity::Finite(a), Intfinity::Finite(b)) => a == b,
            (Intfinity::PosInfinity, Intfinity::PosInfinity) => true,
            (Intfinity::NegInfinity, Intfinity::NegInfinity) => true,
            _ => false,
        }
    }
}

impl<T> Eq for Intfinity<T> where T: Eq {}

impl<T> PartialOrd for Intfinity<T>
where
    T: PartialOrd,
{
     /// Provides partial ordering for `Intfinity` values.
    /// 
    /// - If both values are `Finite`, they are compared using `PartialOrd`.
    /// - `PosInfinity` is greater than any other value except itself, which is considered equal.
    /// - `NegInfinity` is less than any other value except itself, which is considered equal.
    ///
    /// # Example
    /// ```
    /// use intfinity::Intfinity;
    /// let a = Intfinity::new(5);
    /// let b = Intfinity::PosInfinity;
    ///
    /// assert!(a < b);
    /// assert!(b > a);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Intfinity::Finite(a), Intfinity::Finite(b)) => a.partial_cmp(b),
            (Intfinity::PosInfinity, Intfinity::PosInfinity) => Some(std::cmp::Ordering::Equal),
            (Intfinity::NegInfinity, Intfinity::NegInfinity) => Some(std::cmp::Ordering::Equal),
            (Intfinity::PosInfinity, _) => Some(std::cmp::Ordering::Greater),
            (_, Intfinity::PosInfinity) => Some(std::cmp::Ordering::Less),
            (Intfinity::NegInfinity, _) => Some(std::cmp::Ordering::Less),
            (_, Intfinity::NegInfinity) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl<T> Ord for Intfinity<T>
where
    T: Ord,
{
    /// Provides total ordering for `Intfinity` values by using `partial_cmp` and unwrapping the result.
    /// This method assumes that `partial_cmp` will never return `None` for valid comparisons.
    ///
    /// # Example
    /// ```
    /// use intfinity::Intfinity;
    /// let a = Intfinity::new(5);
    /// let b = Intfinity::PosInfinity;
    ///
    /// assert!(a.cmp(&b) == std::cmp::Ordering::Less);
    /// ```
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> Intfinity<T>
where
    T: PartialEq + PartialOrd,
{
    /// Compares two `Intfinity` values in an indeterminate manner.
    ///
    /// - If both values are `Finite`, they are compared using `PartialOrd`.
    /// - If both values are `PosInfinity` or `NegInfinity`, the comparison returns `None`, indicating that they cannot be meaningfully compared.
    /// - If one value is `PosInfinity` and the other is `NegInfinity`, they are ordered accordingly.
    /// - Otherwise, infinities are treated as greater or less than finite values.
    ///
    /// # Example
    /// ```
    /// use intfinity::Intfinity;
    /// let a: Intfinity<i32> = Intfinity::PosInfinity;
    /// let b = Intfinity::NegInfinity;
    ///
    /// match a.indeterminate_cmp(&b) {
    ///     Some(ordering) => println!("Ordering: {:?}", ordering),
    ///     None => println!("Incomparable values"),
    /// }
    /// ```
    pub fn indeterminate_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Intfinity::Finite(a), Intfinity::Finite(b)) => a.partial_cmp(b),
            (Intfinity::PosInfinity, Intfinity::PosInfinity) => None,
            (Intfinity::NegInfinity, Intfinity::NegInfinity) => None,
            (Intfinity::PosInfinity, Intfinity::NegInfinity) => Some(std::cmp::Ordering::Greater),  
            (Intfinity::NegInfinity, Intfinity::PosInfinity) => Some(std::cmp::Ordering::Less),  
            (Intfinity::PosInfinity, _) => Some(std::cmp::Ordering::Greater),
            (_, Intfinity::PosInfinity) => Some(std::cmp::Ordering::Less),
            (Intfinity::NegInfinity, _) => Some(std::cmp::Ordering::Less),
            (_, Intfinity::NegInfinity) => Some(std::cmp::Ordering::Greater),
        }
    }
}