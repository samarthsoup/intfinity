use intfinity::DoubleBoundedInfinity;

#[test]
fn test_display_finite_value() {
    let value = DoubleBoundedInfinity::new(42);
    let output = format!("{}", value);
    assert_eq!(output, "42");
}

#[test]
fn test_display_pos_infinity() {
    let value: DoubleBoundedInfinity<i32> = DoubleBoundedInfinity::PosInfinity;
    let output = format!("{}", value);
    assert_eq!(output, "+infinity");
}

#[test]
fn test_display_neg_infinity() {
    let value: DoubleBoundedInfinity<i32> = DoubleBoundedInfinity::NegInfinity;
    let output = format!("{}", value);
    assert_eq!(output, "-infinity");
}