use intfinity::Intfinity;
use intfinity::traits::Zero;

#[test]
fn test_addition_finite_values() {
    let a = Intfinity::new(10);
    let b = Intfinity::new(20);
    let result = a + b;
    assert_eq!(result, Intfinity::Finite(30));
}

#[test]
fn test_addition_with_overflow() {
    let a = Intfinity::new(i32::MAX);
    let b = Intfinity::new(1);
    let result = a + b;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_addition_with_negative_overflow() {
    let a = Intfinity::new(i32::MIN);
    let b = Intfinity::new(-1);
    let result = a + b;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
#[should_panic(expected = "indeterminate form: +inf + (-inf)")]
fn test_addition_positive_and_negative_infinity() {
    let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
    let neg_inf = Intfinity::NegInfinity;

    let _result = pos_inf + neg_inf;
}

#[test]
fn test_addition_with_positive_infinity() {
    let a = Intfinity::new(-10);
    let result = a + Intfinity::PosInfinity;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_addition_with_negative_infinity() {
    let a = Intfinity::new(10);
    let result = a + Intfinity::NegInfinity;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_subtraction_finite_values() {
    let a = Intfinity::new(20);
    let b = Intfinity::new(10);
    let result = a - b;
    assert_eq!(result, Intfinity::Finite(10));
}

#[test]
fn test_subtraction_with_underflow() {
    let a = Intfinity::new(i32::MIN);
    let b = Intfinity::new(1);
    let result = a - b;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_subtraction_positive_infinity() {
    let a = Intfinity::PosInfinity;
    let b = Intfinity::new(10);
    let result = a - b;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_subtraction_negative_infinity() {
    let a = Intfinity::NegInfinity;
    let b = Intfinity::new(10);
    let result = a - b;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_subtraction_inf_minus_neg_inf() {
    let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
    let neg_inf = Intfinity::NegInfinity;
    let result = pos_inf - neg_inf;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_subtraction_neg_inf_minus_pos_inf() {
    let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;
    let neg_inf = Intfinity::NegInfinity;
    let result = neg_inf - pos_inf;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
#[should_panic(expected = "indeterminate form: inf - inf")]
fn test_subtraction_inf_minus_inf_should_panic_1() {
    let pos_inf: Intfinity<i32> = Intfinity::PosInfinity;

    let _result = pos_inf - pos_inf;
}

#[test]
#[should_panic(expected = "indeterminate form: inf - inf")]
fn test_subtraction_inf_minus_inf_should_panic_2() {
    let neg_inf: Intfinity<i32> = Intfinity::NegInfinity;

    let _result = neg_inf - neg_inf;
}

#[test]
fn test_subtraction_with_overflow() {
    let a = Intfinity::new(i32::MAX);
    let b = Intfinity::new(-1);
    let result = a - b;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_multiplication_finite_values() {
    let a = Intfinity::new(4);
    let b = Intfinity::new(5);
    let result = a * b;
    assert_eq!(result, Intfinity::Finite(20));
}

#[test]
fn test_multiplication_with_overflow() {
    let a = Intfinity::new(i32::MAX);
    let b = Intfinity::new(2);
    let result = a * b;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
#[should_panic(expected = "indefinite form: 0 * inf")]
fn test_multiplication_by_zero_and_infinity() {
    let a = Intfinity::new(0);
    let b = Intfinity::PosInfinity;
    let _result = a * b; 
}

#[test]
#[should_panic(expected = "indefinite form: 0 * -inf")]
fn test_multiplication_by_zero_and_negative_infinity() {
    let a = Intfinity::new(0);
    let b = Intfinity::NegInfinity;
    let _result = a * b;  
}

#[test]
fn test_multiplication_with_pos_infinity_positive() {
    let a = Intfinity::new(4);
    let result = a * Intfinity::PosInfinity;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_multiplication_with_pos_infinity_negative() {
    let a = Intfinity::new(-4);
    let result = a * Intfinity::PosInfinity;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_multiplication_with_neg_infinity_positive() {
    let a = Intfinity::new(4);
    let result = a * Intfinity::NegInfinity;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_multiplication_with_neg_infinity_negative() {
    let a = Intfinity::new(-4);
    let result = a * Intfinity::NegInfinity;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_multiplication_pos_infinity_by_pos_infinity() {
    let result: Intfinity<i32> = Intfinity::PosInfinity * Intfinity::PosInfinity;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_multiplication_neg_infinity_by_neg_infinity() {
    let result: Intfinity<i32> = Intfinity::NegInfinity * Intfinity::NegInfinity;
    assert_eq!(result, Intfinity::PosInfinity);
}

#[test]
fn test_multiplication_pos_infinity_by_neg_infinity() {
    let result: Intfinity<i32> = Intfinity::PosInfinity * Intfinity::NegInfinity;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_multiplication_neg_infinity_by_pos_infinity() {
    let result: Intfinity<i32> = Intfinity::NegInfinity * Intfinity::PosInfinity;
    assert_eq!(result, Intfinity::NegInfinity);
}

#[test]
fn test_multiplication_negative_finite_by_finite() {
    let a = Intfinity::new(-3);
    let b = Intfinity::new(6);
    let result = a * b;
    assert_eq!(result, Intfinity::Finite(-18));
}

#[test]
fn test_division_finite_values() {
    let a = Intfinity::new(10);
    let b = Intfinity::new(2);
    let result = a / b;
    assert_eq!(result, Intfinity::Finite(5));
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_division_by_zero() {
    let a = Intfinity::new(10);
    let b = Intfinity::new(0);
    let _result = a / b;  
}

#[test]
fn test_division_by_pos_infinity() {
    let a = Intfinity::new(10);
    let result = a / Intfinity::PosInfinity;
    assert_eq!(result, Intfinity::Finite(i32::zero())); 
}

#[test]
fn test_division_by_neg_infinity() {
    let a = Intfinity::new(10);
    let result = a / Intfinity::NegInfinity;
    assert_eq!(result, Intfinity::Finite(i32::zero())); 
}

#[test]
fn test_pos_infinity_divided_by_finite() {
    let a = Intfinity::PosInfinity;
    let b = Intfinity::new(2);
    let result = a / b;
    assert_eq!(result, Intfinity::PosInfinity);  
}

#[test]
fn test_neg_infinity_divided_by_finite() {
    let a = Intfinity::NegInfinity;
    let b = Intfinity::new(2);
    let result = a / b;
    assert_eq!(result, Intfinity::NegInfinity);  
}

#[test]
fn test_neg_infinity_divided_by_negative_finite() {
    let a = Intfinity::NegInfinity;
    let b = Intfinity::new(-2);
    let result = a / b;
    assert_eq!(result, Intfinity::PosInfinity);  
}

#[test]
fn test_pos_infinity_divided_by_negative_finite() {
    let a = Intfinity::PosInfinity;
    let b = Intfinity::new(-2);
    let result = a / b;
    assert_eq!(result, Intfinity::NegInfinity); 
}

#[test]
#[should_panic(expected = "indeterminate form: inf / inf")]
fn test_infinity_divided_by_infinity_should_panic_1() {
    let a: Intfinity<i32> = Intfinity::PosInfinity;
    let b: Intfinity<i32> = Intfinity::PosInfinity;
    let _result = a / b; 
}

#[test]
#[should_panic(expected = "indeterminate form: inf / inf")]
fn test_infinity_divided_by_infinity_should_panic_2() {
    let a: Intfinity<i32> = Intfinity::NegInfinity;
    let b: Intfinity<i32> = Intfinity::NegInfinity;
    let _result = a / b;  
}

#[test]
#[should_panic(expected = "indeterminate form: inf / -inf")]
fn test_pos_infinity_divided_by_neg_infinity_should_panic() {
    let a: Intfinity<i32> = Intfinity::PosInfinity;
    let b: Intfinity<i32> = Intfinity::NegInfinity;

    let _result = a / b;   
}


#[test]
#[should_panic(expected = "indeterminate form: inf / -inf")]
fn test_neg_infinity_divided_by_pos_infinity_should_panic() {
    let a: Intfinity<i32> = Intfinity::PosInfinity;
    let b: Intfinity<i32> = Intfinity::NegInfinity;

    let _result = b / a;  
}