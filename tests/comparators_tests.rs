use intfinity::DoubleBoundedInfinity;
use std::cmp::Ordering;

#[test]
fn test_eq_finite() {
    let a = DoubleBoundedInfinity::new(5);
    let b = DoubleBoundedInfinity::new(5);
    let c = DoubleBoundedInfinity::new(10);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_eq_infinity() {
    let pos_inf = DoubleBoundedInfinity::PosInfinity::<i32>;
    let another_pos_inf = DoubleBoundedInfinity::PosInfinity::<i32>;

    assert_eq!(pos_inf, another_pos_inf);
}

#[test]
fn test_ne_infinity() {
    let pos_inf = DoubleBoundedInfinity::PosInfinity::<i32>;
    let neg_inf = DoubleBoundedInfinity::NegInfinity::<i32>;

    assert_ne!(pos_inf, neg_inf);
}

#[test]
fn test_lesser_than_ord_finite() {
    let a = DoubleBoundedInfinity::new(5);
    let b = DoubleBoundedInfinity::new(10);

    assert!(a < b);
}

#[test]
fn test_greater_than_ord_finite() {
    let a = DoubleBoundedInfinity::new(5);
    let b = DoubleBoundedInfinity::new(10);

    assert!(b > a);
}

#[test]
fn test_ord_positive_infinity() {
    let a = DoubleBoundedInfinity::new(5);
    let pos_inf = DoubleBoundedInfinity::PosInfinity::<i32>;

    assert!(pos_inf > a);
}

#[test]
fn test_ord_negative_infinity() {
    let a = DoubleBoundedInfinity::new(5);
    let neg_inf = DoubleBoundedInfinity::NegInfinity::<i32>;

    assert!(neg_inf < a);
}

#[test]
fn test_indeterminate_cmp_finite_finite_greater_than_comparison() {
    let a = DoubleBoundedInfinity::Finite(15);
    let b = DoubleBoundedInfinity::Finite(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_indeterminate_cmp_finite_finite_lesser_than_comparison() {
    let a = DoubleBoundedInfinity::Finite(5);
    let b = DoubleBoundedInfinity::Finite(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_finite_finite_equal_comparison() {
    let a = DoubleBoundedInfinity::Finite(10);
    let b = DoubleBoundedInfinity::Finite(10);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Equal));  
}

#[test]
fn test_indeterminate_cmp_pos_infinity_pos_infinity() {
    let a = DoubleBoundedInfinity::PosInfinity::<i32>;
    let b = DoubleBoundedInfinity::PosInfinity;
    assert_eq!(a.indeterminate_cmp(&b), None);  
}

#[test]
fn test_indeterminate_cmp_neg_infinity_neg_infinity() {
    let a = DoubleBoundedInfinity::NegInfinity::<i32>;
    let b = DoubleBoundedInfinity::NegInfinity;
    assert_eq!(a.indeterminate_cmp(&b), None);  
}

#[test]
fn test_indeterminate_cmp_pos_infinity_neg_infinity() {
    let a = DoubleBoundedInfinity::PosInfinity::<i32>;
    let b = DoubleBoundedInfinity::NegInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_indeterminate_cmp_neg_infinity_pos_infinity() {
    let a = DoubleBoundedInfinity::NegInfinity::<i32>;
    let b = DoubleBoundedInfinity::PosInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_pos_infinity_finite() {
    let a = DoubleBoundedInfinity::PosInfinity;
    let b = DoubleBoundedInfinity::Finite(42);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}

#[test]
fn test_indeterminate_cmp_finite_pos_infinity() {
    let a = DoubleBoundedInfinity::Finite(42);
    let b = DoubleBoundedInfinity::PosInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_neg_infinity_finite() {
    let a = DoubleBoundedInfinity::NegInfinity;
    let b = DoubleBoundedInfinity::Finite(42);
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Less));  
}

#[test]
fn test_indeterminate_cmp_finite_neg_infinity() {
    let a = DoubleBoundedInfinity::Finite(42);
    let b = DoubleBoundedInfinity::NegInfinity;
    assert_eq!(a.indeterminate_cmp(&b), Some(Ordering::Greater));  
}