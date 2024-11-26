use std::ops::{Add, AddAssign, Sub, SubAssign};

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
