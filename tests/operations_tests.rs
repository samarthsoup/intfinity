use intfinity::{SingleInfiniteNumber,DoubleInfiniteNumber};
use intfinity::traits::Zero;

#[test]
fn test_addition_finite_values() {
    let a = DoubleInfiniteNumber::new(10);
    let b = DoubleInfiniteNumber::new(20);
    let result = a + b;
    assert_eq!(result, DoubleInfiniteNumber::Finite(30));
}

#[test]
fn test_addition_with_overflow() {
    let a = DoubleInfiniteNumber::new(i32::MAX);
    let b = DoubleInfiniteNumber::new(1);
    let result = a + b;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_addition_with_negative_overflow() {
    let a = DoubleInfiniteNumber::new(i32::MIN);
    let b = DoubleInfiniteNumber::new(-1);
    let result = a + b;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
#[should_panic(expected = "indeterminate form: +inf + (-inf)")]
fn test_addition_positive_and_negative_infinity() {
    let pos_inf: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let neg_inf = DoubleInfiniteNumber::NegInfinity;

    let _result = pos_inf + neg_inf;
}

#[test]
fn test_addition_with_positive_infinity() {
    let a = DoubleInfiniteNumber::new(-10);
    let result = a + DoubleInfiniteNumber::PosInfinity;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_addition_with_negative_infinity() {
    let a = DoubleInfiniteNumber::new(10);
    let result = a + DoubleInfiniteNumber::NegInfinity;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_subtraction_finite_values() {
    let a = DoubleInfiniteNumber::new(20);
    let b = DoubleInfiniteNumber::new(10);
    let result = a - b;
    assert_eq!(result, DoubleInfiniteNumber::Finite(10));
}

#[test]
fn test_subtraction_with_underflow() {
    let a = DoubleInfiniteNumber::new(i32::MIN);
    let b = DoubleInfiniteNumber::new(1);
    let result = a - b;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_subtraction_positive_infinity() {
    let a = DoubleInfiniteNumber::PosInfinity;
    let b = DoubleInfiniteNumber::new(10);
    let result = a - b;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_subtraction_negative_infinity() {
    let a = DoubleInfiniteNumber::NegInfinity;
    let b = DoubleInfiniteNumber::new(10);
    let result = a - b;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_subtraction_inf_minus_neg_inf() {
    let pos_inf: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let neg_inf = DoubleInfiniteNumber::NegInfinity;
    let result = pos_inf - neg_inf;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_subtraction_neg_inf_minus_pos_inf() {
    let pos_inf: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let neg_inf = DoubleInfiniteNumber::NegInfinity;
    let result = neg_inf - pos_inf;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
#[should_panic(expected = "indeterminate form: inf - inf")]
fn test_subtraction_inf_minus_inf_should_panic_1() {
    let pos_inf: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;

    let _result = pos_inf - pos_inf;
}

#[test]
#[should_panic(expected = "indeterminate form: inf - inf")]
fn test_subtraction_inf_minus_inf_should_panic_2() {
    let neg_inf: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;

    let _result = neg_inf - neg_inf;
}

#[test]
fn test_subtraction_with_overflow() {
    let a = DoubleInfiniteNumber::new(i32::MAX);
    let b = DoubleInfiniteNumber::new(-1);
    let result = a - b;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_multiplication_finite_values() {
    let a = DoubleInfiniteNumber::new(4);
    let b = DoubleInfiniteNumber::new(5);
    let result = a * b;
    assert_eq!(result, DoubleInfiniteNumber::Finite(20));
}

#[test]
fn test_multiplication_with_overflow() {
    let a = DoubleInfiniteNumber::new(i32::MAX);
    let b = DoubleInfiniteNumber::new(2);
    let result = a * b;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
#[should_panic(expected = "indefinite form: 0 * inf")]
fn test_multiplication_by_zero_and_infinity() {
    let a = DoubleInfiniteNumber::new(0);
    let b = DoubleInfiniteNumber::PosInfinity;
    let _result = a * b; 
}

#[test]
#[should_panic(expected = "indefinite form: 0 * -inf")]
fn test_multiplication_by_zero_and_negative_infinity() {
    let a = DoubleInfiniteNumber::new(0);
    let b = DoubleInfiniteNumber::NegInfinity;
    let _result = a * b;  
}

#[test]
fn test_multiplication_with_pos_infinity_positive() {
    let a = DoubleInfiniteNumber::new(4);
    let result = a * DoubleInfiniteNumber::PosInfinity;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_multiplication_with_pos_infinity_negative() {
    let a = DoubleInfiniteNumber::new(-4);
    let result = a * DoubleInfiniteNumber::PosInfinity;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_multiplication_with_neg_infinity_positive() {
    let a = DoubleInfiniteNumber::new(4);
    let result = a * DoubleInfiniteNumber::NegInfinity;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_multiplication_with_neg_infinity_negative() {
    let a = DoubleInfiniteNumber::new(-4);
    let result = a * DoubleInfiniteNumber::NegInfinity;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_multiplication_pos_infinity_by_pos_infinity() {
    let result: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity * DoubleInfiniteNumber::PosInfinity;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_multiplication_neg_infinity_by_neg_infinity() {
    let result: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity * DoubleInfiniteNumber::NegInfinity;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);
}

#[test]
fn test_multiplication_pos_infinity_by_neg_infinity() {
    let result: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity * DoubleInfiniteNumber::NegInfinity;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_multiplication_neg_infinity_by_pos_infinity() {
    let result: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity * DoubleInfiniteNumber::PosInfinity;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);
}

#[test]
fn test_multiplication_negative_finite_by_finite() {
    let a = DoubleInfiniteNumber::new(-3);
    let b = DoubleInfiniteNumber::new(6);
    let result = a * b;
    assert_eq!(result, DoubleInfiniteNumber::Finite(-18));
}

#[test]
fn test_division_finite_values() {
    let a = DoubleInfiniteNumber::new(10);
    let b = DoubleInfiniteNumber::new(2);
    let result = a / b;
    assert_eq!(result, DoubleInfiniteNumber::Finite(5));
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_division_by_zero() {
    let a = DoubleInfiniteNumber::new(10);
    let b = DoubleInfiniteNumber::new(0);
    let _result = a / b;  
}

#[test]
fn test_division_by_pos_infinity() {
    let a = DoubleInfiniteNumber::new(10);
    let result = a / DoubleInfiniteNumber::PosInfinity;
    assert_eq!(result, DoubleInfiniteNumber::Finite(i32::zero())); 
}

#[test]
fn test_division_by_neg_infinity() {
    let a = DoubleInfiniteNumber::new(10);
    let result = a / DoubleInfiniteNumber::NegInfinity;
    assert_eq!(result, DoubleInfiniteNumber::Finite(i32::zero())); 
}

#[test]
fn test_pos_infinity_divided_by_finite() {
    let a = DoubleInfiniteNumber::PosInfinity;
    let b = DoubleInfiniteNumber::new(2);
    let result = a / b;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);  
}

#[test]
fn test_neg_infinity_divided_by_finite() {
    let a = DoubleInfiniteNumber::NegInfinity;
    let b = DoubleInfiniteNumber::new(2);
    let result = a / b;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity);  
}

#[test]
fn test_neg_infinity_divided_by_negative_finite() {
    let a = DoubleInfiniteNumber::NegInfinity;
    let b = DoubleInfiniteNumber::new(-2);
    let result = a / b;
    assert_eq!(result, DoubleInfiniteNumber::PosInfinity);  
}

#[test]
fn test_pos_infinity_divided_by_negative_finite() {
    let a = DoubleInfiniteNumber::PosInfinity;
    let b = DoubleInfiniteNumber::new(-2);
    let result = a / b;
    assert_eq!(result, DoubleInfiniteNumber::NegInfinity); 
}

#[test]
#[should_panic(expected = "indeterminate form: inf / inf")]
fn test_infinity_divided_by_infinity_should_panic_1() {
    let a: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let b: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let _result = a / b; 
}

#[test]
#[should_panic(expected = "indeterminate form: inf / inf")]
fn test_infinity_divided_by_infinity_should_panic_2() {
    let a: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;
    let b: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;
    let _result = a / b;  
}

#[test]
#[should_panic(expected = "indeterminate form: inf / -inf")]
fn test_pos_infinity_divided_by_neg_infinity_should_panic() {
    let a: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let b: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;

    let _result = a / b;   
}


#[test]
#[should_panic(expected = "indeterminate form: inf / -inf")]
fn test_neg_infinity_divided_by_pos_infinity_should_panic() {
    let a: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    let b: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;

    let _result = b / a;  
}

#[test]
fn test_addition_finite_values_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let b = SingleInfiniteNumber::new(20);
    let result = a + b;
    assert_eq!(result, SingleInfiniteNumber::Finite(30));
}

#[test]
fn test_addition_with_overflow_single() {
    let a = SingleInfiniteNumber::new(u32::MAX);
    let b = SingleInfiniteNumber::new(1);
    let result = a + b;
    assert_eq!(result, SingleInfiniteNumber::Infinity);
}

#[test]
fn test_addition_with_infinity_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let result = a + SingleInfiniteNumber::Infinity;
    assert_eq!(result, SingleInfiniteNumber::Infinity);
}

#[test]
fn test_subtraction_finite_values_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(20);
    let b = SingleInfiniteNumber::new(10);
    let result = a - b;
    assert_eq!(result, SingleInfiniteNumber::Finite(10));
}

#[test]
fn test_subtraction_with_infinity_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let result = a - SingleInfiniteNumber::Infinity;
    assert_eq!(result, SingleInfiniteNumber::Finite(0));
}

#[test]
fn test_multiplication_finite_values_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(4);
    let b = SingleInfiniteNumber::new(5);
    let result = a * b;
    assert_eq!(result, SingleInfiniteNumber::Finite(20));
}

#[test]
fn test_multiplication_with_infinity_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(4);
    let result = a * SingleInfiniteNumber::Infinity;
    assert_eq!(result, SingleInfiniteNumber::Infinity);
}

#[test]
#[should_panic(expected = "indeterminate form: 0 * inf")]
fn test_multiplication_by_zero_and_infinity_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(0);
    let _result = a * SingleInfiniteNumber::Infinity;
}

#[test]
fn test_division_finite_values_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let b = SingleInfiniteNumber::new(2);
    let result = a / b;
    assert_eq!(result, SingleInfiniteNumber::Finite(5));
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_division_by_zero_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let _result = a / SingleInfiniteNumber::new(0);
}

#[test]
fn test_division_by_infinity_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let result = a / SingleInfiniteNumber::Infinity;
    assert_eq!(result, SingleInfiniteNumber::Finite(0));
}

#[test]
#[should_panic(expected = "indeterminate form: inf / inf")]
fn test_infinity_divided_by_infinity_should_panic_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::Infinity;
    let _result = a / SingleInfiniteNumber::Infinity;
}
