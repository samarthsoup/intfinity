use crate::traits::{Zero, Negate, Unsigned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoubleBoundedInfinity<T> {
    /// Negative infinity (-inf).
    NegInfinity,
    /// A finite value of type T.
    Finite(T),
    /// Positive infinity (+inf).
    PosInfinity,
}

impl<T> DoubleBoundedInfinity<T>
where
    T: Copy,
{
    /// Creates a new `DoubleBoundedInfinity` instance with a finite value.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::DoubleBoundedInfinity;
    /// let value = DoubleBoundedInfinity::new(42);
    /// assert_eq!(value, DoubleBoundedInfinity::Finite(42));
    /// ```
    pub fn new(value: T) -> Self {
        DoubleBoundedInfinity::Finite(value)
    }
}

impl<T> DoubleBoundedInfinity<T>
where
    T: Zero + PartialEq,
{
    /// Checks if the `DoubleBoundedInfinity` value is zero.
    ///
    /// Returns `true` if the value is `Finite` and equals zero, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::DoubleBoundedInfinity;
    /// let value = DoubleBoundedInfinity::new(0);
    /// assert!(value.is_zero());
    ///
    /// let inf: DoubleBoundedInfinity<i32> = DoubleBoundedInfinity::PosInfinity;
    /// assert!(!inf.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        match self {
            DoubleBoundedInfinity::Finite(val) => val.is_zero(),
            _ => false,
        }
    }
}

impl<T> DoubleBoundedInfinity<T>
where
    T: Copy + Negate,
{
    /// Returns the negated value of the `DoubleBoundedInfinity` instance.
    ///
    /// If the value is finite, it will return the result of applying the `negate()` function, as defined by the user for the type `T`.
    /// If the value is `PosInfinity`, it will return `NegInfinity`, and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::DoubleBoundedInfinity;
    /// let value = DoubleBoundedInfinity::new(42);
    /// let negated = value.negate_double_bounded_infinity();
    /// assert_eq!(negated, DoubleBoundedInfinity::Finite(-42));
    ///
    /// let pos_inf = DoubleBoundedInfinity::PosInfinity::<i32>;
    /// let neg_inf = pos_inf.negate_double_bounded_infinity();
    /// assert_eq!(neg_inf, DoubleBoundedInfinity::NegInfinity);
    /// ```
    pub fn negate_double_bounded_infinity(self) -> Self {
        match self {
            DoubleBoundedInfinity::Finite(value) => DoubleBoundedInfinity::Finite(value.negate()), 
            DoubleBoundedInfinity::PosInfinity => DoubleBoundedInfinity::NegInfinity,  
            DoubleBoundedInfinity::NegInfinity => DoubleBoundedInfinity::PosInfinity,  
        }
    }
}

impl<T> std::fmt::Display for DoubleBoundedInfinity<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoubleBoundedInfinity::Finite(val) => write!(f, "{}", val),
            DoubleBoundedInfinity::PosInfinity => write!(f, "+infinity"),
            DoubleBoundedInfinity::NegInfinity => write!(f, "-infinity"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SingleBoundedInfinity<T: Unsigned> {
    /// A finite value of type T.
    Finite(T),
    /// Positive infinity (+inf).
    Infinity,
}

impl<T> SingleBoundedInfinity<T>
where
    T: Copy + Unsigned,
{
    /// Creates a new `SingleBoundedInfinity` instance with a finite value.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::SingleBoundedInfinity;
    /// let value = SingleBoundedInfinity::new(42);
    /// assert_eq!(value, SingleBoundedInfinity::Finite(42));
    /// ```
    pub fn new(value: T) -> Self {
        SingleBoundedInfinity::Finite(value)
    }
}

impl<T> std::fmt::Display for SingleBoundedInfinity<T>
where
    T: std::fmt::Display + Unsigned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleBoundedInfinity::Finite(val) => write!(f, "{}", val),
            SingleBoundedInfinity::Infinity => write!(f, "+infinity"),
        }
    }
}
