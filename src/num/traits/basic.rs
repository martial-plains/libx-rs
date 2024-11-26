use std::{
    hash::Hash,
    ops::{
        Add, AddAssign, BitOr, BitOrAssign, BitXor, Mul, MulAssign, Neg, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

pub trait AdditiveArithmetic:
    Sized + Add<Output = Self> + Add + AddAssign + Sub<Output = Self> + Sub + SubAssign
{
    const ZERO: Self;

    const ONE: Self;
}

impl AdditiveArithmetic for i8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for i16 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for i32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for i64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for i128 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for u8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for u16 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for u32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for u64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for u128 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl AdditiveArithmetic for f32 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}

impl AdditiveArithmetic for f64 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
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

pub trait SignedNumeric: Numeric + Neg {
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

pub trait BinaryInteger:
    Hash + Numeric + Rem + BitXor + BitOr + RemAssign + BitOrAssign + Shl + Shr + ShlAssign + ShrAssign
{
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self);

    fn is_multiple_of(&self, other: Self) -> bool;

    #[must_use]
    fn signum(&self) -> Self;

    fn is_signed() -> bool;

    fn bit_width(&self) -> usize;

    fn trailing_zero_bit_count(&self) -> usize;
}

impl BinaryInteger for u8 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        Self::from(*self > 0)
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u16 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        Self::from(*self > 0)
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u32 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        Self::from(*self > 0)
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u64 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        Self::from(*self > 0)
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u128 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        Self::from(*self > 0)
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for i8 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        if *self < 0 {
            -1
        } else {
            Self::from(*self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.abs().count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i16 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        if *self < 0 {
            -1
        } else {
            Self::from(*self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.abs().count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i32 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        if *self < 0 {
            -1
        } else {
            Self::from(*self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.abs().count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i64 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        if *self < 0 {
            -1
        } else {
            Self::from(*self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.abs().count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i128 {
    fn quotient_and_remainder_dividing_by(&self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    fn is_multiple_of(&self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    fn signum(&self) -> Self {
        if *self < 0 {
            -1
        } else {
            Self::from(*self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn bit_width(&self) -> usize {
        if *self == 0 {
            0
        } else {
            self.abs().count_ones() as usize
        }
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

pub trait FixedWidthInteger: BinaryInteger {
    #[must_use]
    fn big_endian(&self) -> Self;

    #[must_use]
    fn byte_swapped(&self) -> Self;

    fn leading_zero_bit_count(&self) -> usize;

    #[must_use]
    fn little_endian(&self) -> Self;

    fn nonzero_bit_count(&self) -> usize;

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    fn max() -> Self;

    fn min() -> Self;
}

impl FixedWidthInteger for u8 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for u16 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for u32 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for u64 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for u128 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for i8 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.unsigned_abs().count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for i16 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.unsigned_abs().count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for i32 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.unsigned_abs().count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for i64 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.unsigned_abs().count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

impl FixedWidthInteger for i128 {
    #[must_use]
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    #[must_use]
    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

    #[must_use]
    fn little_endian(&self) -> Self {
        self.to_le()
    }

    fn nonzero_bit_count(&self) -> usize {
        self.unsigned_abs().count_ones() as usize
    }

    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }

    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_div(rhs)
        }
    }

    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_mul(rhs)
    }

    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        if rhs == 0 {
            // Handle division by zero for remainder, returning zero and a boolean flag indicating overflow.
            (0, true)
        } else {
            self.overflowing_rem(rhs)
        }
    }

    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }

    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

pub trait SignedInteger: BinaryInteger + SignedNumeric {}

impl SignedInteger for i8 {}

impl SignedInteger for i16 {}

impl SignedInteger for i32 {}

impl SignedInteger for i64 {}

impl SignedInteger for i128 {}
