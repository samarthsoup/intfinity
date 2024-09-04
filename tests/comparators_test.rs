use intfinity::Intfinity;

#[test]
fn test_eq_finite() {
    let a = Intfinity::new(5);
    let b = Intfinity::new(5);
    let c = Intfinity::new(10);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_eq_infinity() {
    let pos_inf = Intfinity::PosInfinity::<i32>;
    let neg_inf = Intfinity::NegInfinity::<i32>;
    let another_pos_inf = Intfinity::PosInfinity::<i32>;

    assert_eq!(pos_inf, another_pos_inf);
    assert_ne!(pos_inf, neg_inf);
}

#[test]
fn test_ord_finite() {
    let a = Intfinity::new(5);
    let b = Intfinity::new(10);

    assert!(a < b);
    assert!(b > a);
}

#[test]
fn test_ord_infinity() {
    let a = Intfinity::new(5);
    let pos_inf = Intfinity::PosInfinity::<i32>;
    let neg_inf = Intfinity::NegInfinity::<i32>;

    assert!(pos_inf > a);
    assert!(neg_inf < a);
}

#[test]
fn test_indeterminate_cmp_infinities() {
    let pos_inf = Intfinity::PosInfinity::<i32>;
    let another_pos_inf = Intfinity::PosInfinity::<i32>;

    assert_eq!(pos_inf.indeterminate_cmp(&another_pos_inf), None);

    let neg_inf = Intfinity::NegInfinity::<i32>;

    assert_eq!(pos_inf.indeterminate_cmp(&neg_inf), Some(std::cmp::Ordering::Greater));
    assert_eq!(neg_inf.indeterminate_cmp(&pos_inf), Some(std::cmp::Ordering::Less));
}