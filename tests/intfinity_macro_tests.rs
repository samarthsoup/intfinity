use ::intfinity::SingleInfiniteNumber;
use intfinity::{DoubleInfiniteNumber, intfinity};

#[test]
fn test_single_bounded_infinity_with_value() {
    let finite: SingleInfiniteNumber<u32> = intfinity!(42, single_bound);
    assert_eq!(finite, SingleInfiniteNumber::Finite(42));
}

#[test]
fn test_single_bounded_infinity_infinity() {
    let infinity: SingleInfiniteNumber<u32> = intfinity!(inf, single_bound);
    assert_eq!(infinity, SingleInfiniteNumber::Infinity);
}

#[test]
fn test_double_bounded_infinity_with_value() {
    let finite = intfinity!(42, double_bound);
    assert_eq!(finite, DoubleInfiniteNumber::Finite(42));
}

#[test]
fn test_double_bounded_infinity_pos_infinity() {
    let pos_inf: DoubleInfiniteNumber<i32> = intfinity!(inf, double_bound);
    assert_eq!(pos_inf, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_double_bounded_infinity_neg_infinity() {
    let neg_inf: DoubleInfiniteNumber<i32> = intfinity!(-inf, double_bound);
    assert_eq!(neg_inf, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_double_bounded_infinity_default_pos_infinity() {
    let pos_inf: DoubleInfiniteNumber<i32> = intfinity!(inf);
    assert_eq!(pos_inf, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_double_bounded_infinity_default_neg_infinity() {
    let neg_inf: DoubleInfiniteNumber<i32> = intfinity!(-inf);
    assert_eq!(neg_inf, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_double_bounded_infinity_default_finite() {
    let finite = intfinity!(42);
    assert_eq!(finite, DoubleInfiniteNumber::Finite(42));
}