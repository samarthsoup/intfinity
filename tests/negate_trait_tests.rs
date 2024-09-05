use intfinity::DoubleInfiniteNumber;

#[test]
fn test_negate_finite_positive() {
    let a = DoubleInfiniteNumber::Finite(10);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleInfiniteNumber::Finite(-10));
}

#[test]
fn test_negate_finite_negative() {
    let a = DoubleInfiniteNumber::Finite(-10);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleInfiniteNumber::Finite(10));
}

#[test]
fn test_negate_pos_infinity() {
    let a = DoubleInfiniteNumber::PosInfinity::<i32>;
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_negate_neg_infinity() {
    let a = DoubleInfiniteNumber::NegInfinity::<i32>;
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_negate_zero() {
    let a = DoubleInfiniteNumber::Finite(0);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleInfiniteNumber::Finite(0));
}

#[test]
fn test_negate_large_value() {
    let a = DoubleInfiniteNumber::Finite(100000);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleInfiniteNumber::Finite(-100000));
}
