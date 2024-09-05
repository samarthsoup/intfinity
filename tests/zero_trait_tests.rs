use intfinity::{traits::{Unsigned, Zero}, DoubleInfiniteNumber, SingleInfiniteNumber};

#[derive(Clone,Copy,PartialEq)]
struct IntWrapper(i32);

impl Zero for IntWrapper {
    fn zero() -> Self {
        IntWrapper(0)
    }

    fn is_zero(&self) -> bool {
        let near_zero_threshold = 2;  
        self.0.abs() <= near_zero_threshold  
    }
}

#[derive(Clone,Copy,PartialEq)]
struct UIntWrapper(u32);

impl Zero for UIntWrapper {
    fn zero() -> Self {
        UIntWrapper(0)
    }

    fn is_zero(&self) -> bool {
        let near_zero_threshold = 2;  
        self.0 <= near_zero_threshold  
    }
}

impl Unsigned for UIntWrapper {}

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
    let near_zero = DoubleInfiniteNumber::new(IntWrapper(1));
    assert!(near_zero.is_zero());  
}

#[test]
fn test_custom_is_zero_for_integers_outside_zero_threshold() {
    let far_from_zero = DoubleInfiniteNumber::new(IntWrapper(3));
    assert!(!far_from_zero.is_zero());  
}

#[test]
fn test_zero_for_finite_singleinfinity() {
    let zero_value: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(0);
    assert!(zero_value.is_zero());
}

#[test]
fn test_non_zero_for_finite_singleinfinity() {
    let non_zero_value: SingleInfiniteNumber<u32> = SingleInfiniteNumber::new(42);
    assert!(!non_zero_value.is_zero());
}

#[test]
fn test_infinity_is_not_zero_singleinfinity() {
    let infinity: SingleInfiniteNumber<u32> = SingleInfiniteNumber::Infinity;
    assert!(!infinity.is_zero());
}

#[test]
fn test_custom_is_zero_for_integers_within_threshold_singleinfinity() {
    let near_zero = SingleInfiniteNumber::new(UIntWrapper(1));
    assert!(near_zero.is_zero());
}

#[test]
fn test_custom_is_zero_for_integers_outside_threshold_singleinfinity() {
    let far_from_zero = SingleInfiniteNumber::new(UIntWrapper(3));
    assert!(!far_from_zero.is_zero());
}