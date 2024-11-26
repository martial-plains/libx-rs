use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait AdditiveArithmetic:
    Sized + Add<Output = Self> + Add + AddAssign + Sub<Output = Self> + Sub + SubAssign
{
    const ZERO: Self;
}

impl AdditiveArithmetic for i8 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for i16 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for i32 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for i64 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for i128 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for u8 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for u16 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for u32 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for u64 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for u128 {
    const ZERO: Self = 0;
}

impl AdditiveArithmetic for f32 {
    const ZERO: Self = 0.0;
}

impl AdditiveArithmetic for f64 {
    const ZERO: Self = 0.0;
}

pub trait Numeric: AdditiveArithmetic + Mul + MulAssign {}

impl Numeric for i8 {}

impl Numeric for i16 {}

impl Numeric for i32 {}

impl Numeric for i64 {}

impl Numeric for i128 {}

impl Numeric for u8 {}

impl Numeric for u16 {}

impl Numeric for u32 {}

impl Numeric for u64 {}

impl Numeric for u128 {}

impl Numeric for f32 {}

impl Numeric for f64 {}

pub trait SignedNumeric: Numeric {
    fn negate(&mut self);
}

impl SignedNumeric for i8 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for i16 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for i32 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for i64 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for i128 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for f32 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for f64 {
    fn negate(&mut self) {
        *self = self.neg();
    }
}
