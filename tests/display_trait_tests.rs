use intfinity::Intfinity;

#[test]
fn test_display_finite_value() {
    let value = Intfinity::new(42);
    let output = format!("{}", value);
    assert_eq!(output, "42");
}

#[test]
fn test_display_pos_infinity() {
    let value: Intfinity<i32> = Intfinity::PosInfinity;
    let output = format!("{}", value);
    assert_eq!(output, "+infinity");
}

#[test]
fn test_display_neg_infinity() {
    let value: Intfinity<i32> = Intfinity::NegInfinity;
    let output = format!("{}", value);
    assert_eq!(output, "-infinity");
}