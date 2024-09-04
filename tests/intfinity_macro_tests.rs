use intfinity::{DoubleBoundedInfinity, intfinity};

#[test]
fn test_finite_value() {
    let finite_value = intfinity!(42);
    assert_eq!(finite_value, DoubleBoundedInfinity::Finite(42));
}

#[test]
fn test_pos_infinity() {
    let pos_inf: DoubleBoundedInfinity<i32> = intfinity!(+inf);
    assert_eq!(pos_inf, DoubleBoundedInfinity::PosInfinity);
}

#[test]
fn test_neg_infinity() {
    let neg_inf: DoubleBoundedInfinity<i32> = intfinity!(-inf);
    assert_eq!(neg_inf, DoubleBoundedInfinity::NegInfinity);
}