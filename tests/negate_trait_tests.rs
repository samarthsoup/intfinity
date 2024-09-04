use intfinity::Intfinity;

#[test]
fn test_negate_finite_positive() {
    let a = Intfinity::Finite(10);
    let result = a.negate_intfinity();
    assert_eq!(result, Intfinity::Finite(-10));
}

#[test]
fn test_negate_finite_negative() {
    let a = Intfinity::Finite(-10);
    let result = a.negate_intfinity();
    assert_eq!(result, Intfinity::Finite(10));
}

#[test]
fn test_negate_pos_infinity() {
    let a = Intfinity::PosInfinity::<i32>;
    let result = a.negate_intfinity();
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_negate_neg_infinity() {
    let a = Intfinity::NegInfinity::<i32>;
    let result = a.negate_intfinity();
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_negate_zero() {
    let a = Intfinity::Finite(0);
    let result = a.negate_intfinity();
    assert_eq!(result, Intfinity::Finite(0));
}

#[test]
fn test_negate_large_value() {
    let a = Intfinity::Finite(100000);
    let result = a.negate_intfinity();
    assert_eq!(result, Intfinity::Finite(-100000));
}
