use intfinity::DoubleBoundedInfinity;

#[test]
fn test_negate_finite_positive() {
    let a = DoubleBoundedInfinity::Finite(10);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleBoundedInfinity::Finite(-10));
}

#[test]
fn test_negate_finite_negative() {
    let a = DoubleBoundedInfinity::Finite(-10);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleBoundedInfinity::Finite(10));
}

#[test]
fn test_negate_pos_infinity() {
    let a = DoubleBoundedInfinity::PosInfinity::<i32>;
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleBoundedInfinity::NegInfinity);
}

#[test]
fn test_negate_neg_infinity() {
    let a = DoubleBoundedInfinity::NegInfinity::<i32>;
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleBoundedInfinity::PosInfinity);
}

#[test]
fn test_negate_zero() {
    let a = DoubleBoundedInfinity::Finite(0);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleBoundedInfinity::Finite(0));
}

#[test]
fn test_negate_large_value() {
    let a = DoubleBoundedInfinity::Finite(100000);
    let result = a.negate_double_bounded_infinity();
    assert_eq!(result, DoubleBoundedInfinity::Finite(-100000));
}
