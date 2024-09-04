//hi

pub mod traits;
pub mod intfinity;
pub mod operations;
pub mod numeric_impls;

pub use intfinity::Intfinity;

#[macro_export]
macro_rules! intfinity {
    (+inf) => {
        Intfinity::PosInfinity
    };
    (-inf) => {
        Intfinity::NegInfinity
    };
    ($val:expr) => {
        Intfinity::new($val)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finite_value() {
        let finite_value = intfinity!(42);
        assert_eq!(finite_value, Intfinity::Finite(42));
    }

    #[test]
    fn test_pos_infinity() {
        let pos_inf: Intfinity<i32> = intfinity!(+inf);
        assert_eq!(pos_inf, Intfinity::PosInfinity);
    }

    #[test]
    fn test_neg_infinity() {
        let neg_inf: Intfinity<i32> = intfinity!(-inf);
        assert_eq!(neg_inf, Intfinity::NegInfinity);
    }
}
