pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool where Self:Sized, Self: PartialEq {
        *self == Self::zero()
    }
}

pub trait Negate {
    fn negate(self) -> Self;
}


pub trait CheckedAdd: Sized {
    fn checked_add(self, other: Self) -> Option<Self>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(self, other: Self) -> Option<Self>;
}

pub trait CheckedMul: Sized {
    fn checked_mul(self, other: Self) -> Option<Self>;
}


pub trait CheckedDiv: Sized {
    fn checked_div(self, other: Self) -> Option<Self>;
}

pub trait Unsigned {}