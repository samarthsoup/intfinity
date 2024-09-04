//hi

pub mod traits;
pub mod intfinity;
pub mod operations;
pub mod numeric_impls;

pub use intfinity::{DoubleBoundedInfinity, SingleBoundedInfinity};

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
    (inf) => {
        SingleBoundedInfinity::Infinity
    };
    (inf, single_bound) => {
        SingleBoundedInfinity::Infinity
    };
    ($val:expr, single_bound) => {
        SingleBoundedInfinity::new($val)
    };
    (+inf, double_bound) => {
        DoubleBoundedInfinity::PosInfinity
    };
    (-inf, double_bound) => {
        DoubleBoundedInfinity::NegInfinity
    };
    ($val:expr, double_bound) => {
        DoubleBoundedInfinity::new($val)
    };
}