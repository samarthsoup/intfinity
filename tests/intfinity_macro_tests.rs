use ::intfinity::SingleBoundedInfinity;
use intfinity::{DoubleBoundedInfinity, intfinity};

#[test]
fn test_single_bounded_infinity_with_value() {
    let finite = intfinity!(42, single_bound);
    assert_eq!(finite, SingleBoundedInfinity::Finite(42));
}

#[test]
fn test_single_bounded_infinity_infinity() {
    let infinity = intfinity!(inf, single_bound);
    assert_eq!(infinity, SingleBoundedInfinity::Infinity);
}

#[test]
fn test_double_bounded_infinity_default_infinity() {
    let infinity = intfinity!(inf);
    assert_eq!(infinity, SingleBoundedInfinity::Infinity);
}

#[test]
fn test_double_bounded_infinity_with_value() {
    let finite = intfinity!(42, double_bound);
    assert_eq!(finite, DoubleBoundedInfinity::Finite(42));
}

#[test]
fn test_double_bounded_infinity_pos_infinity() {
    let pos_inf = intfinity!(+inf, double_bound);
    assert_eq!(pos_inf, DoubleBoundedInfinity::PosInfinity);
}

#[test]
fn test_double_bounded_infinity_neg_infinity() {
    let neg_inf = intfinity!(-inf, double_bound);
    assert_eq!(neg_inf, DoubleBoundedInfinity::NegInfinity);
}

#[test]
fn test_double_bounded_infinity_default_pos_infinity() {
    let pos_inf = intfinity!(+inf);
    assert_eq!(pos_inf, DoubleBoundedInfinity::PosInfinity);
}

#[test]
fn test_double_bounded_infinity_default_neg_infinity() {
    let neg_inf = intfinity!(-inf);
    assert_eq!(neg_inf, DoubleBoundedInfinity::NegInfinity);
}

#[test]
fn test_double_bounded_infinity_default_finite() {
    let finite = intfinity!(42);
    assert_eq!(finite, DoubleBoundedInfinity::Finite(42));
}