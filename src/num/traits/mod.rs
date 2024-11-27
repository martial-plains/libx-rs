use core::{
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

pub trait UnsignedInteger: BinaryInteger {}

impl UnsignedInteger for u8 {}

impl UnsignedInteger for u16 {}

impl UnsignedInteger for u32 {}

impl UnsignedInteger for u64 {}

impl UnsignedInteger for u128 {}

pub trait FloatingPoint: SignedNumeric {
    type Exponent: SignedInteger;

    #[must_use]
    fn ceil(self) -> Self;

    #[must_use]
    fn floor(self) -> Self;

    #[must_use]
    fn fract(self) -> Self;

    #[must_use]
    fn trunc(self) -> Self;

    fn exponent(self) -> Self::Exponent;

    fn floating_point_class(&self) -> FloatingPointClassification;

    fn is_canonical(&self) -> bool;

    fn is_finite(&self) -> bool;

    fn is_infinite(&self) -> bool;

    fn is_nan(&self) -> bool;

    fn is_normal(&self) -> bool;

    fn is_signaling_nan(&self) -> bool;

    fn is_subnormal(&self) -> bool;

    fn is_zero(&self) -> bool;

    #[must_use]
    fn next_down(self) -> Self;

    #[must_use]
    fn next_up(self) -> Self;

    fn sign(&self) -> FloatingPointSign;

    #[must_use]
    fn significand(self) -> Self;

    #[must_use]
    fn ulp(self) -> Self;

    fn add_product(&mut self, lhs: Self, rhs: Self);

    #[must_use]
    fn adding_product(self, lhs: Self, rhs: Self) -> Self;

    fn form_remainder(&mut self, other: Self);

    fn form_square_root(&mut self);

    fn form_truncating_remainder(&mut self, other: Self);

    fn is_equal_to(&self, other: Self) -> bool;

    fn is_less_than(&self, other: Self) -> bool;

    fn is_less_than_or_equal_to(&self, other: Self) -> bool;

    fn is_totally_ordered_below_or_equal_to(&self, other: Self) -> bool;

    #[must_use]
    fn remainder(self, other: Self) -> Self;

    fn round(&mut self);

    fn round_with(&mut self, rule: FloatingPointRoundingRule);

    #[must_use]
    fn rounded(self) -> Self;

    #[must_use]
    fn rounded_with(self, rule: FloatingPointRoundingRule) -> Self;

    #[must_use]
    fn square_root(self) -> Self;

    #[must_use]
    fn truncating_remainder(self, other: Self) -> Self;

    fn greatest_finite_magnitude() -> Self;

    fn infinity() -> Self;

    fn least_nonzero_magnitude() -> Self;

    fn least_normal_magnitude() -> Self;

    fn nan() -> Self;

    fn pi() -> Self;

    fn radix() -> Self;

    fn signaling_nan() -> Self;

    fn ulp_of_one() -> Self;

    fn maximum(x: Self, y: Self) -> Self;

    fn maximum_magnitude(x: Self, y: Self) -> Self;

    fn minimum(x: Self, y: Self) -> Self;

    fn minimum_magnitude(x: Self, y: Self) -> Self;
}

impl FloatingPoint for f32 {
    type Exponent = i32;

    fn ceil(self) -> Self {
        if self.is_nan() {
            return self;
        }

        if self.is_infinite() {
            return self;
        }

        if self >= 0.0 {
            return (self as Self::Exponent) as Self
                + if self == (self as Self::Exponent) as Self {
                    0.0
                } else {
                    1.0
                };
        }

        let truncated = (self as Self::Exponent) as Self;
        return truncated;
    }

    fn floor(self) -> Self {
        if self.is_nan() {
            return self;
        }

        if self.is_infinite() {
            return self;
        }

        if self >= 0.0 {
            return (self as Self::Exponent) as Self;
        }

        let truncated = (self as Self::Exponent) as Self;
        if self == truncated {
            return truncated;
        }

        truncated - 1.0
    }

    fn fract(self) -> Self {
        self - self.floor()
    }

    fn trunc(self) -> Self {
        self - self.fract()
    }

    #[allow(clippy::cast_possible_wrap)]
    fn exponent(self) -> Self::Exponent {
        self.to_bits() as i32 >> 23 & 0xFF
    }

    fn floating_point_class(&self) -> FloatingPointClassification {
        if self.is_nan() {
            if self.is_signaling_nan() {
                FloatingPointClassification::SignalingNaN
            } else {
                FloatingPointClassification::QuietNaN
            }
        } else if self.is_infinite() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeInfinity
            } else {
                FloatingPointClassification::PositiveInfinity
            }
        } else if self.is_zero() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeZero
            } else {
                FloatingPointClassification::PositiveZero
            }
        } else if self.is_normal() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeNormal
            } else {
                FloatingPointClassification::PositiveNormal
            }
        } else if self.is_subnormal() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeSubnormal
            } else {
                FloatingPointClassification::PositiveSubnormal
            }
        } else {
            panic!("Unhandled case for floating point class")
        }
    }

    fn is_canonical(&self) -> bool {
        !self.is_nan()
    }

    fn is_finite(&self) -> bool {
        self.is_normal() || self.is_zero()
    }

    fn is_infinite(&self) -> bool {
        Self::is_infinite(*self)
    }

    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }

    fn is_normal(&self) -> bool {
        Self::is_normal(*self)
    }

    fn is_signaling_nan(&self) -> bool {
        false
    }

    fn is_subnormal(&self) -> bool {
        Self::is_subnormal(*self)
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn next_down(self) -> Self {
        let mut bits = self.to_bits();

        if self.is_nan() {
            return self;
        } else if self.is_infinite() {
            return if self.is_sign_negative() {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        } else if self == 0.0 {
            return if self.is_sign_negative() {
                -Self::ZERO
            } else {
                Self::ZERO
            };
        }

        if self.is_sign_negative() {
            bits += 1;
        } else {
            bits -= 1;
        }

        Self::from_bits(bits)
    }

    fn next_up(self) -> Self {
        let mut bits = self.to_bits();

        if self.is_nan() {
            return self;
        } else if self.is_infinite() {
            return if self.is_sign_negative() {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        } else if self == 0.0 {
            return if self.is_sign_negative() {
                -Self::ZERO
            } else {
                Self::ZERO
            };
        }

        if self.is_sign_negative() {
            bits -= 1;
        } else {
            bits += 1;
        }

        Self::from_bits(bits)
    }

    fn sign(&self) -> FloatingPointSign {
        if self.is_sign_negative() {
            FloatingPointSign::Minus
        } else {
            FloatingPointSign::Plus
        }
    }

    fn significand(self) -> Self {
        if self == 0.0 {
            return 0.0;
        }

        let raw_bits = self.to_bits();
        let exponent = (raw_bits >> 23) & 0xFF;
        let significand = raw_bits & 0x007F_FFFF;

        if exponent == 0 {
            return Self::from_bits(significand);
        }

        let normalized_significand = (1u32 << 23) | significand;

        Self::from_bits(normalized_significand)
    }

    fn ulp(self) -> Self {
        Self::EPSILON
    }

    fn add_product(&mut self, lhs: Self, rhs: Self) {
        *self += lhs * rhs;
    }

    fn adding_product(self, lhs: Self, rhs: Self) -> Self {
        self + lhs * rhs
    }

    fn form_remainder(&mut self, other: Self) {
        *self = self.remainder(other);
    }

    fn form_square_root(&mut self) {
        *self = self.square_root();
    }

    fn form_truncating_remainder(&mut self, other: Self) {
        *self = self.truncating_remainder(other);
    }

    fn is_equal_to(&self, other: Self) -> bool {
        (self - other).abs() < 0.1
    }

    fn is_less_than(&self, other: Self) -> bool {
        self < &other
    }

    fn is_less_than_or_equal_to(&self, other: Self) -> bool {
        self <= &other
    }

    fn is_totally_ordered_below_or_equal_to(&self, other: Self) -> bool {
        self.is_finite() && other.is_finite()
    }

    fn remainder(self, other: Self) -> Self {
        Self::rem(self, other)
    }

    fn round(&mut self) {
        *self = unsafe { self.to_int_unchecked::<u64>() } as Self;
    }

    fn round_with(&mut self, rule: FloatingPointRoundingRule) {
        *self = match rule {
            FloatingPointRoundingRule::AwayFromZero => {
                if *self > 0.0 {
                    self.ceil()
                } else if *self < 0.0 {
                    self.floor()
                } else {
                    *self
                }
            }
            FloatingPointRoundingRule::Down => self.floor(),
            FloatingPointRoundingRule::ToNearestOrAwayFromZero => {
                if self.is_nan() {
                    *self
                } else if (self.fract() - 0.5).abs() < 0.1 || (self.fract() - -0.5).abs() < 0.1 {
                    if *self > 0.0 {
                        self.ceil()
                    } else if *self < 0.0 {
                        self.floor()
                    } else {
                        *self
                    }
                } else {
                    self.rounded()
                }
            }
            FloatingPointRoundingRule::ToNearestOrEven => {
                if self.is_nan() {
                    *self
                } else {
                    self.rounded()
                }
            }
            FloatingPointRoundingRule::TowardZero => self.trunc(),
            FloatingPointRoundingRule::Up => self.ceil(),
        };
    }

    fn rounded(self) -> Self {
        unsafe { self.to_int_unchecked::<u64>() as Self }
    }

    fn rounded_with(self, rule: FloatingPointRoundingRule) -> Self {
        match rule {
            FloatingPointRoundingRule::AwayFromZero => {
                if self > 0.0 {
                    self.ceil()
                } else if self < 0.0 {
                    self.floor()
                } else {
                    self
                }
            }
            FloatingPointRoundingRule::Down => self.floor(),
            FloatingPointRoundingRule::ToNearestOrAwayFromZero => {
                if self.is_nan() {
                    self
                } else if (self.fract() - 0.5).abs() < 0.1 || (self.fract() - 0.5).abs() < 0.1 {
                    if self > 0.0 {
                        self.ceil()
                    } else if self < 0.0 {
                        self.floor()
                    } else {
                        self
                    }
                } else {
                    self.rounded()
                }
            }
            FloatingPointRoundingRule::ToNearestOrEven => {
                if self.is_nan() {
                    self
                } else {
                    self.rounded()
                }
            }
            FloatingPointRoundingRule::TowardZero => self.trunc(),
            FloatingPointRoundingRule::Up => self.ceil(),
        }
    }

    fn square_root(self) -> Self {
        if self < 0.0 {
            return Self::NAN;
        }

        if self == 0.0 {
            return 0.0;
        }

        let mut guess = self / 2.0;
        let mut last_guess = 0.0;

        let tolerance = 1e-6;

        while (guess - last_guess).abs() > tolerance {
            last_guess = guess;
            guess = (guess + self / guess) / 2.0;
        }

        guess
    }

    fn truncating_remainder(self, other: Self) -> Self {
        let truncated_quotient = (self / other).trunc();
        self - (other * truncated_quotient)
    }

    fn greatest_finite_magnitude() -> Self {
        Self::MAX
    }

    fn infinity() -> Self {
        Self::INFINITY
    }

    fn least_nonzero_magnitude() -> Self {
        Self::EPSILON
    }

    fn least_normal_magnitude() -> Self {
        Self::MIN_POSITIVE
    }

    fn nan() -> Self {
        Self::NAN
    }

    fn pi() -> Self {
        core::f32::consts::PI
    }

    fn radix() -> Self {
        2.0
    }

    fn signaling_nan() -> Self {
        Self::NAN
    }

    fn ulp_of_one() -> Self {
        Self::EPSILON
    }

    fn maximum(x: Self, y: Self) -> Self {
        x.max(y)
    }

    fn maximum_magnitude(x: Self, y: Self) -> Self {
        x.abs().max(y.abs())
    }

    fn minimum(x: Self, y: Self) -> Self {
        x.min(y)
    }

    fn minimum_magnitude(x: Self, y: Self) -> Self {
        x.abs().min(y.abs())
    }
}

impl FloatingPoint for f64 {
    type Exponent = i64;

    fn ceil(self) -> Self {
        if self.is_nan() {
            return self;
        }

        if self.is_infinite() {
            return self;
        }

        if self >= 0.0 {
            return (self as Self::Exponent) as Self
                + if self == (self as Self::Exponent) as Self {
                    0.0
                } else {
                    1.0
                };
        }

        (self as Self::Exponent) as Self
    }

    fn floor(self) -> Self {
        if self.is_nan() {
            return self;
        }

        if self.is_infinite() {
            return self;
        }

        if self >= 0.0 {
            return (self as Self::Exponent) as Self;
        }

        let truncated = (self as Self::Exponent) as Self;
        if self == truncated {
            return truncated;
        }

        truncated - 1.0
    }

    fn fract(self) -> Self {
        self - self.floor()
    }

    fn trunc(self) -> Self {
        self - self.fract()
    }

    #[allow(clippy::cast_possible_wrap)]
    fn exponent(self) -> Self::Exponent {
        self.to_bits() as Self::Exponent >> 23 & 0xFF
    }

    fn floating_point_class(&self) -> FloatingPointClassification {
        if self.is_nan() {
            if self.is_signaling_nan() {
                FloatingPointClassification::SignalingNaN
            } else {
                FloatingPointClassification::QuietNaN
            }
        } else if self.is_infinite() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeInfinity
            } else {
                FloatingPointClassification::PositiveInfinity
            }
        } else if self.is_zero() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeZero
            } else {
                FloatingPointClassification::PositiveZero
            }
        } else if self.is_normal() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeNormal
            } else {
                FloatingPointClassification::PositiveNormal
            }
        } else if self.is_subnormal() {
            if self.is_sign_negative() {
                FloatingPointClassification::NegativeSubnormal
            } else {
                FloatingPointClassification::PositiveSubnormal
            }
        } else {
            panic!("Unhandled case for floating point class")
        }
    }

    fn is_canonical(&self) -> bool {
        !self.is_nan()
    }

    fn is_finite(&self) -> bool {
        self.is_normal() || self.is_zero()
    }

    fn is_infinite(&self) -> bool {
        Self::is_infinite(*self)
    }

    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }

    fn is_normal(&self) -> bool {
        Self::is_normal(*self)
    }

    fn is_signaling_nan(&self) -> bool {
        false
    }

    fn is_subnormal(&self) -> bool {
        Self::is_subnormal(*self)
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn next_down(self) -> Self {
        let mut bits = self.to_bits();

        if self.is_nan() {
            return self;
        } else if self.is_infinite() {
            return if self.is_sign_negative() {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        } else if self == 0.0 {
            return if self.is_sign_negative() {
                -Self::ZERO
            } else {
                Self::ZERO
            };
        }

        // For non-zero numbers, manipulate the raw bits.
        if self.is_sign_negative() {
            // Negative number: increment the bit representation.
            bits += 1;
        } else {
            // Positive number: decrement the bit representation.
            bits -= 1;
        }

        // Convert the bits back into an f32.
        Self::from_bits(bits)
    }

    fn next_up(self) -> Self {
        // Convert the f32 to raw bits.
        let mut bits = self.to_bits();

        // If the number is positive, we add 1 (move up).
        // If the number is negative, we subtract 1 (move up in the negative direction).
        if self.is_nan() {
            return self; // NaN can't go next up.
        } else if self.is_infinite() {
            return if self.is_sign_negative() {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        } else if self == 0.0 {
            return if self.is_sign_negative() {
                -Self::ZERO
            } else {
                Self::ZERO
            };
        }

        // For non-zero numbers, manipulate the raw bits.
        if self.is_sign_negative() {
            // Negative number: decrement the bit representation.
            bits -= 1;
        } else {
            // Positive number: increment the bit representation.
            bits += 1;
        }

        // Convert the bits back into an f32.
        Self::from_bits(bits)
    }

    fn sign(&self) -> FloatingPointSign {
        if self.is_sign_negative() {
            FloatingPointSign::Minus
        } else {
            FloatingPointSign::Plus
        }
    }

    fn significand(self) -> Self {
        self.fract()
    }

    fn ulp(self) -> Self {
        Self::EPSILON
    }

    fn add_product(&mut self, lhs: Self, rhs: Self) {
        *self = lhs * rhs;
    }

    fn adding_product(self, lhs: Self, rhs: Self) -> Self {
        lhs * rhs
    }

    fn form_remainder(&mut self, other: Self) {
        *self = self.remainder(other);
    }

    fn form_square_root(&mut self) {
        *self = self.square_root();
    }

    fn form_truncating_remainder(&mut self, other: Self) {
        *self = self.truncating_remainder(other);
    }

    fn is_equal_to(&self, other: Self) -> bool {
        (self - other).abs() < 0.1
    }

    fn is_less_than(&self, other: Self) -> bool {
        self < &other
    }

    fn is_less_than_or_equal_to(&self, other: Self) -> bool {
        self <= &other
    }

    fn is_totally_ordered_below_or_equal_to(&self, other: Self) -> bool {
        self.is_finite() && other.is_finite()
    }

    fn remainder(self, other: Self) -> Self {
        Self::rem(self, other)
    }

    fn round(&mut self) {
        *self = Self::rounded(*self);
    }

    fn round_with(&mut self, rule: FloatingPointRoundingRule) {
        *self = match rule {
            FloatingPointRoundingRule::AwayFromZero => {
                if *self > 0.0 {
                    self.ceil()
                } else if *self < 0.0 {
                    self.floor()
                } else {
                    *self
                }
            }
            FloatingPointRoundingRule::Down => self.floor(),
            FloatingPointRoundingRule::ToNearestOrAwayFromZero => {
                if self.is_nan() {
                    *self // NaN remains unchanged
                } else if (self.fract() - 0.5).abs() < 0.1 || (self.fract() - -0.5).abs() < 0.1 {
                    // Handle halfway cases by rounding away from zero
                    if *self > 0.0 {
                        self.ceil() // Round up for positive numbers
                    } else if *self < 0.0 {
                        self.floor() // Round down for negative numbers
                    } else {
                        *self // No change for zero
                    }
                } else {
                    self.rounded() // Standard rounding
                }
            }
            FloatingPointRoundingRule::ToNearestOrEven => {
                if self.is_nan() {
                    *self
                } else {
                    self.rounded()
                }
            }
            FloatingPointRoundingRule::TowardZero => self.trunc(),
            FloatingPointRoundingRule::Up => self.ceil(),
        };
    }

    fn rounded(self) -> Self {
        unsafe { self.to_int_unchecked::<u64>() as Self }
    }

    fn rounded_with(self, rule: FloatingPointRoundingRule) -> Self {
        match rule {
            FloatingPointRoundingRule::AwayFromZero => {
                if self > 0.0 {
                    self.ceil()
                } else if self < 0.0 {
                    self.floor()
                } else {
                    self
                }
            }
            FloatingPointRoundingRule::Down => self.floor(),
            FloatingPointRoundingRule::ToNearestOrAwayFromZero => {
                if self.is_nan() {
                    self // NaN remains unchanged
                } else if (self.fract() - 0.5).abs() < 0.1 || (self.fract() - 0.5).abs() < 0.1 {
                    // Handle halfway cases by rounding away from zero
                    if self > 0.0 {
                        self.ceil() // Round up for positive numbers
                    } else if self < 0.0 {
                        self.floor() // Round down for negative numbers
                    } else {
                        self // No change for zero
                    }
                } else {
                    self.rounded() // Standard rounding
                }
            }
            FloatingPointRoundingRule::ToNearestOrEven => {
                if self.is_nan() {
                    self
                } else {
                    self.rounded()
                }
            }
            FloatingPointRoundingRule::TowardZero => self.trunc(),
            FloatingPointRoundingRule::Up => self.ceil(),
        }
    }

    fn square_root(self) -> Self {
        if self < 0.0 {
            return Self::NAN;
        }
        if self == 0.0 {
            return 0.0;
        }

        let mut guess = self / 2.0;
        let mut last_guess = 0.0;

        while (guess - last_guess).abs() > 0.0001 {
            last_guess = guess;
            guess = (guess + self / guess) / 2.0;
        }

        guess
    }

    fn truncating_remainder(self, other: Self) -> Self {
        let truncated_quotient = (self / other).trunc();
        self - (other * truncated_quotient)
    }

    fn greatest_finite_magnitude() -> Self {
        Self::MAX
    }

    fn infinity() -> Self {
        Self::INFINITY
    }

    fn least_nonzero_magnitude() -> Self {
        Self::EPSILON
    }

    fn least_normal_magnitude() -> Self {
        Self::MIN_POSITIVE
    }

    fn nan() -> Self {
        Self::NAN
    }

    fn pi() -> Self {
        core::f64::consts::PI
    }

    fn radix() -> Self {
        2.0
    }

    fn signaling_nan() -> Self {
        Self::NAN
    }

    fn ulp_of_one() -> Self {
        Self::EPSILON
    }

    fn maximum(x: Self, y: Self) -> Self {
        x.max(y)
    }

    fn maximum_magnitude(x: Self, y: Self) -> Self {
        x.abs().max(y.abs())
    }

    fn minimum(x: Self, y: Self) -> Self {
        x.min(y)
    }

    fn minimum_magnitude(x: Self, y: Self) -> Self {
        x.abs().min(y.abs())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FloatingPointClassification {
    NegativeInfinity,
    NegativeNormal,
    NegativeSubnormal,
    NegativeZero,
    PositiveInfinity,
    PositiveNormal,
    PositiveSubnormal,
    PositiveZero,
    QuietNaN,
    SignalingNaN,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FloatingPointSign {
    Minus,
    Plus,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FloatingPointRoundingRule {
    AwayFromZero,
    Down,
    ToNearestOrAwayFromZero,
    ToNearestOrEven,
    TowardZero,
    Up,
}
