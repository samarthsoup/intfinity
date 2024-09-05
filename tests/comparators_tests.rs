use intfinity::{SingleInfiniteNumber, DoubleInfiniteNumber};
use std::cmp::Ordering;

#[test]
fn test_eq_finite() {
    let a = DoubleInfiniteNumber::new(5);
    let b = DoubleInfiniteNumber::new(5);
    let c = DoubleInfiniteNumber::new(10);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_eq_infinity() {
    let pos_inf = DoubleInfiniteNumber::PosInfinity::<i32>;
    let another_pos_inf = DoubleInfiniteNumber::PosInfinity::<i32>;

    assert_eq!(pos_inf, another_pos_inf);
}

#[test]
fn test_ne_infinity() {
    let pos_inf = DoubleInfiniteNumber::PosInfinity::<i32>;
    let neg_inf = DoubleInfiniteNumber::NegInfinity::<i32>;

    assert_ne!(pos_inf, neg_inf);
}

#[test]
fn test_lesser_than_ord_finite() {
    let a = DoubleInfiniteNumber::new(5);
    let b = DoubleInfiniteNumber::new(10);

    assert!(a < b);
}

#[test]
fn test_greater_than_ord_finite() {
    let a = DoubleInfiniteNumber::new(5);
    let b = DoubleInfiniteNumber::new(10);

    assert!(b > a);
}

#[test]
fn test_ord_positive_infinity() {
    let a = DoubleInfiniteNumber::new(5);
    let pos_inf = DoubleInfiniteNumber::PosInfinity::<i32>;

    assert!(pos_inf > a);
}

#[test]
fn test_ord_negative_infinity() {
    let a = DoubleInfiniteNumber::new(5);
    let neg_inf = DoubleInfiniteNumber::NegInfinity::<i32>;

    assert!(neg_inf < a);
}

#[test]
fn test_indeterminate_cmp_finite_finite_greater_than_comparison() {
    let a = DoubleInfiniteNumber::Finite(15);
    let b = DoubleInfiniteNumber::Finite(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_indeterminate_cmp_finite_finite_lesser_than_comparison() {
    let a = DoubleInfiniteNumber::Finite(5);
    let b = DoubleInfiniteNumber::Finite(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_finite_finite_equal_comparison() {
    let a = DoubleInfiniteNumber::Finite(10);
    let b = DoubleInfiniteNumber::Finite(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Equal));  
}

#[test]
fn test_indeterminate_cmp_pos_infinity_pos_infinity() {
    let a = DoubleInfiniteNumber::PosInfinity::<i32>;
    let b = DoubleInfiniteNumber::PosInfinity;
    assert_eq!(a.indeterminate_cmp(&b), None);  
}

#[test]
fn test_indeterminate_cmp_neg_infinity_neg_infinity() {
    let a = DoubleInfiniteNumber::NegInfinity::<i32>;
    let b = DoubleInfiniteNumber::NegInfinity;
    assert_eq!(a.indeterminate_cmp(&b), None);  
}

#[test]
fn test_indeterminate_cmp_pos_infinity_neg_infinity() {
    let a = DoubleInfiniteNumber::PosInfinity::<i32>;
    let b = DoubleInfiniteNumber::NegInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_indeterminate_cmp_neg_infinity_pos_infinity() {
    let a = DoubleInfiniteNumber::NegInfinity::<i32>;
    let b = DoubleInfiniteNumber::PosInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_pos_infinity_finite() {
    let a = DoubleInfiniteNumber::PosInfinity;
    let b = DoubleInfiniteNumber::Finite(42);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_indeterminate_cmp_finite_pos_infinity() {
    let a = DoubleInfiniteNumber::Finite(42);
    let b = DoubleInfiniteNumber::PosInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_neg_infinity_finite() {
    let a = DoubleInfiniteNumber::NegInfinity;
    let b = DoubleInfiniteNumber::Finite(42);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_finite_neg_infinity() {
    let a = DoubleInfiniteNumber::Finite(42);
    let b = DoubleInfiniteNumber::NegInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_eq_finite_single_1() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(5);
    let b = SingleInfiniteNumber::new(5);
    assert_eq!(a, b);
}

#[test]
fn test_eq_finite_single_2() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(5);
    let c = SingleInfiniteNumber::new(10);
    assert_ne!(a, c);
}

#[test]
fn test_eq_infinity_single() {
    let inf = SingleInfiniteNumber::Infinity::<u32>;
    let another_inf = SingleInfiniteNumber::Infinity::<u32>;
    assert_eq!(inf, another_inf);
}

#[test]
fn test_ord_finite_lesser_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(5);
    let b = SingleInfiniteNumber::new(10);
    assert!(a < b);
}

#[test]
fn test_ord_finite_greater_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let b = SingleInfiniteNumber::new(5);
    assert!(a > b);
}

#[test]
fn test_ord_finite_equal_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let b = SingleInfiniteNumber::new(10);
    assert!(a == b);
}

#[test]
fn test_ord_infinity_greater_than_finite_single() {
    let a = SingleInfiniteNumber::new(5);
    let inf = SingleInfiniteNumber::Infinity::<u32>;
    assert!(inf > a);
}

#[test]
fn test_ord_finite_lesser_than_infinity_single() {
    let a = SingleInfiniteNumber::new(5);
    let inf = SingleInfiniteNumber::Infinity::<u32>;
    assert!(a < inf);
}

#[test]
fn test_indeterminate_cmp_finite_greater_than_finite_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(15);
    let b = SingleInfiniteNumber::new(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));
}

#[test]
fn test_indeterminate_cmp_finite_lesser_than_finite_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(5);
    let b = SingleInfiniteNumber::new(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));
}

#[test]
fn test_indeterminate_cmp_finite_equal_single() {
    let a: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(10);
    let b = SingleInfiniteNumber::new(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Equal));
}

#[test]
fn test_indeterminate_cmp_infinity_infinity_single() {
    let inf = SingleInfiniteNumber::Infinity::<u32>;
    assert_eq!(inf.indeterminate_cmp(&inf), None);
}

#[test]
fn test_indeterminate_cmp_infinity_greater_than_finite_single() {
    let inf = SingleInfiniteNumber::Infinity::<u32>;
    let finite = SingleInfiniteNumber::new(42);
    assert_eq!(inf.indeterminate_cmp(&finite), Some(Ordering::Greater));
}

#[test]
fn test_indeterminate_cmp_finite_lesser_than_infinity_single() {
    let inf = SingleInfiniteNumber::Infinity::<u32>;
    let finite = SingleInfiniteNumber::new(42);
    assert_eq!(finite.indeterminate_cmp(&inf), Some(Ordering::Less));
}
