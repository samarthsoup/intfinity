//hi
#![no_std]
pub mod traits;
pub mod intfinity;
pub mod operations;
pub mod numeric_impls;

pub use intfinity::{DoubleInfiniteNumber, SingleInfiniteNumber};

#[macro_export]
macro_rules! intfinity {
    (inf) => {
        DoubleInfiniteNumber::PosInfinity
    };
    (-inf) => {
        DoubleInfiniteNumber::NegInfinity
    };
    ($val:expr) => {
        DoubleInfiniteNumber::new($val)
    };
    (inf, single_bound) => {
        SingleInfiniteNumber::Infinity
    };
    ($val:expr, single_bound) => {
        SingleInfiniteNumber::new($val)
    };
    (inf, double_bound) => {
        DoubleInfiniteNumber::PosInfinity
    };
    (-inf, double_bound) => {
        DoubleInfiniteNumber::NegInfinity
    };
    ($val:expr, double_bound) => {
        DoubleInfiniteNumber::new($val)
    };
}