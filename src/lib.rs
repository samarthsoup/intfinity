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