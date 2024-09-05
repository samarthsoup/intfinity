use intfinity::DoubleInfiniteNumber;

#[test]
fn test_display_finite_value() {
    let value = DoubleInfiniteNumber::new(42);
    let output = format!("{}", value);
    assert_eq!(output, "42");
}

#[test]
fn test_display_pos_infinity() {
    let value: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let output = format!("{}", value);
    assert_eq!(output, "+infinity");
}

#[test]
fn test_display_neg_infinity() {
    let value: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;
    let output = format!("{}", value);
    assert_eq!(output, "-infinity");
}