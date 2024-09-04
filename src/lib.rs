//hi

pub mod traits;
pub mod intfinity;
pub mod operations;
pub mod numeric_impls;

pub use intfinity::DoubleBoundedInfinity;

#[macro_export]
macro_rules! intfinity {
    (+inf) => {
        DoubleBoundedInfinity::PosInfinity
    };
    (-inf) => {
        DoubleBoundedInfinity::NegInfinity
    };
    ($val:expr) => {
        DoubleBoundedInfinity::new($val)
    };
}