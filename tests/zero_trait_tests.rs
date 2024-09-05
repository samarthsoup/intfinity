use intfinity::{traits::Zero, DoubleInfiniteNumber};

#[derive(Clone,Copy,PartialEq)]
struct MyInt(i32);

impl Zero for MyInt {
    fn zero() -> Self {
        MyInt(0)
    }

    fn is_zero(&self) -> bool {
        let near_zero_threshold = 2;  
        self.0.abs() <= near_zero_threshold  
    }
}

#[test]
fn test_zero_for_finite_intfinity() {
    let zero_value = DoubleInfiniteNumber::new(0);
    assert_eq!(zero_value.is_zero(), true);  
}

#[test]
fn test_non_zero_for_finite_intfinity() {
    let non_zero_value = DoubleInfiniteNumber::new(42);
    assert!(!non_zero_value.is_zero());  
}

#[test]
fn test_pos_infinity_is_not_zero() {
    let pos_infinity: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::PosInfinity;
    assert!(!pos_infinity.is_zero());  
}

#[test]
fn test_neg_infinity_is_not_zero() {
    let neg_infinity: DoubleInfiniteNumber<i32> = DoubleInfiniteNumber::NegInfinity;
    assert!(!neg_infinity.is_zero());  
}

#[test]
fn test_custom_is_zero_for_integers_within_zero_threshold() {
    let near_zero = DoubleInfiniteNumber::new(MyInt(1));
    assert!(near_zero.is_zero());  
}

#[test]
fn test_custom_is_zero_for_integers_outside_zero_threshold() {
    let far_from_zero = DoubleInfiniteNumber::new(MyInt(3));
    assert!(!far_from_zero.is_zero());  
}