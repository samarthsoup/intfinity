use intfinity::{SingleInfiniteNumber, DoubleInfiniteNumber};

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

#[test]
fn test_display_finite_value_single() {
    let value: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(42);
    let output = format!("{}", value);
    assert_eq!(output, "42");
}

#[test]
fn test_display_infinity_single() {
    let value: SingleInfiniteNumber<u32> = SingleInfiniteNumber::Infinity;
    let output = format!("{}", value);
    assert_eq!(output, "+infinity");
}