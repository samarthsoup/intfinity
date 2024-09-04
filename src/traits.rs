/// A trait that defines the concept of zero for a type.
///
/// This trait is useful for types where zero is a meaningful value, such as numeric types.
/// It provides a method to get the zero value and to check if a value is zero.
///
/// # Example
///
/// ```
/// use intfinity::traits::Zero;
/// 
/// #[derive(Debug, PartialEq)]
/// struct MyNumber(i32);
///
/// impl Zero for MyNumber {
///     fn zero() -> Self {
///         MyNumber(0)
///     }
///
///     fn is_zero(&self) -> bool {
///         self.0 == 0
///     }
/// }
///
/// let x = MyNumber::zero();
/// assert!(x.is_zero());
/// ```
pub trait Zero {
    /// Returns the zero value of the type.
    fn zero() -> Self;
    /// Checks whether the value is zero.
    fn is_zero(&self) -> bool where Self:Sized, Self: PartialEq {
        *self == Self::zero()
    }
}

/// A trait for types that support negation.
///
/// This trait provides a method to get the negated value of a type. It is primarily used for numeric types
/// where negation is defined.
///
/// # Example
///
/// ```
/// use intfinity::traits::Negate;
/// 
/// #[derive(Debug, PartialEq)]
/// struct MyNumber(i32);
///
/// impl Negate for MyNumber {
///     fn negate(self) -> Self {
///         MyNumber(-self.0)
///     }
/// }
///
/// let x = MyNumber(10);
/// let y = x.negate();
/// assert_eq!(y.0, -10);
/// ```
pub trait Negate {
    /// Returns the negated value of `self`.
    fn negate(self) -> Self;
}

/// A trait for types that support checked addition.
///
/// This trait provides a method to perform addition that checks for overflow.
/// If the addition would overflow, it returns `None`; otherwise, it returns `Some(result)`.
///
/// # Example
///
/// ```
/// use intfinity::traits::CheckedAdd;
/// 
/// #[derive(Debug, PartialEq)]
/// struct MyNumber(i32);
///
/// impl CheckedAdd for MyNumber {
///     fn checked_add(self, other: Self) -> Option<Self> {
///         self.0.checked_add(other.0).map(MyNumber)
///     }
/// }
///
/// let x = MyNumber(10);
/// let y = MyNumber(20);
/// assert_eq!(x.checked_add(y), Some(MyNumber(30)));
/// ```
pub trait CheckedAdd: Sized {
    /// Performs addition, returning `None` if overflow occurs.
    fn checked_add(self, other: Self) -> Option<Self>;
}

/// A trait for types that support checked subtraction.
///
/// This trait provides a method to perform subtraction that checks for overflow.
/// If the subtraction would overflow, it returns `None`; otherwise, it returns `Some(result)`.
///
/// # Example
///
/// ```
/// use intfinity::traits::CheckedSub;
/// 
/// #[derive(Debug, PartialEq)]
/// struct MyNumber(i32);
///
/// impl CheckedSub for MyNumber {
///     fn checked_sub(self, other: Self) -> Option<Self> {
///         self.0.checked_sub(other.0).map(MyNumber)
///     }
/// }
///
/// let x = MyNumber(10);
/// let y = MyNumber(5);
/// assert_eq!(x.checked_sub(y), Some(MyNumber(5)));
/// ```
pub trait CheckedSub: Sized {
    /// Performs subtraction, returning `None` if overflow occurs.
    fn checked_sub(self, other: Self) -> Option<Self>;
}

/// A trait for types that support checked multiplication.
///
/// This trait provides a method to perform multiplication that checks for overflow.
/// If the multiplication would overflow, it returns `None`; otherwise, it returns `Some(result)`.
///
/// # Example
///
/// ```
/// use intfinity::traits::CheckedMul;
/// 
/// #[derive(Debug, PartialEq)]
/// struct MyNumber(i32);
///
/// impl CheckedMul for MyNumber {
///     fn checked_mul(self, other: Self) -> Option<Self> {
///         self.0.checked_mul(other.0).map(MyNumber)
///     }
/// }
///
/// let x = MyNumber(10);
/// let y = MyNumber(20);
/// assert_eq!(x.checked_mul(y), Some(MyNumber(200)));
/// ```
pub trait CheckedMul: Sized {
    /// Performs multiplication, returning `None` if overflow occurs.
    fn checked_mul(self, other: Self) -> Option<Self>;
}

/// A trait for types that support checked division.
///
/// This trait provides a method to perform division that checks for division by zero or overflow.
/// If the division would result in an error, it returns `None`; otherwise, it returns `Some(result)`.
///
/// # Example
///
/// ```
/// use intfinity::traits::CheckedDiv;
/// 
/// #[derive(Debug, PartialEq)]
/// struct MyNumber(i32);
///
/// impl CheckedDiv for MyNumber {
///     fn checked_div(self, other: Self) -> Option<Self> {
///         if other.0 == 0 {
///             None
///         } else {
///             self.0.checked_div(other.0).map(MyNumber)
///         }
///     }
/// }
///
/// let x = MyNumber(20);
/// let y = MyNumber(5);
/// assert_eq!(x.checked_div(y), Some(MyNumber(4)));
/// ```
pub trait CheckedDiv: Sized {
    /// Performs division, returning `None` if division by zero or overflow occurs.
    fn checked_div(self, other: Self) -> Option<Self>;
}

pub trait Unsigned {}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}