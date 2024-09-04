use crate::traits::{Zero, Negate};

/// An enum that represents a value that can either be finite, positive infinity,
/// or negative infinity.
///
/// The `Intfinity` type is generic over any type `T` that implements the necessary traits.
/// This can be used to represent mathematical infinity for various numeric types.
/// This type supports both positive and negative infinity as bounds.
/// While it can be used to represent an infinity bound on only one side (e.g., just positive infinity),
/// the type does not inherently enforce this. Consequently, even if you intend to use only the positive infinity bound,
/// the negative infinity bound remains accessible and usable, though the behavior may not be strictly guaranteed by the type system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Intfinity<T> {
    /// Negative infinity (-inf).
    NegInfinity,
    /// A finite value of type T.
    Finite(T),
    /// Positive infinity (+inf).
    PosInfinity,
}

impl<T> Intfinity<T>
where
    T: Copy,
{
    /// Creates a new `Intfinity` instance with a finite value.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::Intfinity;
    /// let value = Intfinity::new(42);
    /// assert_eq!(value, Intfinity::Finite(42));
    /// ```
    pub fn new(value: T) -> Self {
        Intfinity::Finite(value)
    }
}

impl<T> Intfinity<T>
where
    T: Zero + PartialEq,
{
    /// Checks if the `Intfinity` value is zero.
    ///
    /// Returns `true` if the value is `Finite` and equals zero, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::Intfinity;
    /// let value = Intfinity::new(0);
    /// assert!(value.is_zero());
    ///
    /// let inf: Intfinity<i32> = Intfinity::PosInfinity;
    /// assert!(!inf.is_zero());
    /// ```
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
    /// Returns the negated value of the `Intfinity` instance.
    ///
    /// If the value is finite, it will return the result of applying the `negate()` function, as defined by the user for the type `T`.
    /// If the value is `PosInfinity`, it will return `NegInfinity`, and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use intfinity::Intfinity;
    /// let value = Intfinity::new(42);
    /// let negated = value.negate_intfinity();
    /// assert_eq!(negated, Intfinity::Finite(-42));
    ///
    /// let pos_inf = Intfinity::PosInfinity::<i32>;
    /// let neg_inf = pos_inf.negate_intfinity();
    /// assert_eq!(neg_inf, Intfinity::NegInfinity);
    /// ```
    pub fn negate_intfinity(self) -> Self {
        match self {
            Intfinity::Finite(value) => Intfinity::Finite(value.negate()), 
            Intfinity::PosInfinity => Intfinity::NegInfinity,  
            Intfinity::NegInfinity => Intfinity::PosInfinity,  
        }
    }
}

impl<T> std::fmt::Display for Intfinity<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Intfinity::Finite(val) => write!(f, "{}", val),
            Intfinity::PosInfinity => write!(f, "+infinity"),
            Intfinity::NegInfinity => write!(f, "-infinity"),
        }
    }
}