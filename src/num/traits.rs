use core::{
    hash::Hash,
    mem,
    ops::{
        Add, AddAssign, BitOr, BitOrAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Rem,
        RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

/// A trait for types that support additive arithmetic operations.
///
/// The `AdditiveArithmetic` trait provides the necessary operations for additive arithmetic on scalar
/// values, such as integers, floating-point numbers, or custom types. It allows you to write generic
/// methods that work with any type that supports addition and subtraction, and it includes constants
/// for the additive identity (`ZERO`) and multiplicative identity (`ONE`).
///
/// Types that implement `AdditiveArithmetic` must provide implementations for:
/// - The addition (`+`), subtraction (`-`), and their corresponding assignment variants (`+=`, `-=`),
/// - The `ZERO` constant, which represents the additive identity, and
/// - The `ONE` constant, which represents the multiplicative identity (if relevant for the type).
///
/// # Examples
///
/// ```rust
/// use libx::num::traits::AdditiveArithmetic;
///
/// // Define a sum method for any sequence of elements that implement AdditiveArithmetic
/// fn sum<T>(sequence: &[T]) -> T
/// where
///     T: AdditiveArithmetic + std::clone::Clone
/// {
///     sequence.iter().cloned().fold(T::ZERO, |acc, x| acc + x)
/// }
///
/// let arr = [1.1, 2.2, 3.3, 4.4, 5.5];
/// let total = sum(&arr);
/// println!("Sum: {}", total); // Output: Sum: 16.5
/// ```
pub trait AdditiveArithmetic:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialEq
    + PartialOrd<Self>
{
    /// The additive identity for the type (e.g., `0` for integers or floats).
    const ZERO: Self;

    /// The multiplicative identity for the type (e.g., `1` for integers or floats).
    const ONE: Self;
}

impl AdditiveArithmetic for isize {
    const ZERO: Self = 0;

    const ONE: Self = 1;
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

impl AdditiveArithmetic for usize {
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

/// The `Numeric` trait provides a common interface for scalar numeric types, enabling arithmetic operations
///
/// such as addition and multiplication. It combines the functionality of both the `AdditiveArithmetic` and
/// `Mul` traits to support basic arithmetic on numeric types. This trait is useful for writing generic methods
/// that can operate on any numeric type in the standard library, such as integers and floating-point numbers.
///
/// Types that implement `Numeric` can be used in contexts where both addition and multiplication are required.
/// This trait is designed to be used as a constraint in generic functions or methods to provide arithmetic capabilities
/// on sequences or collections of numeric values.
///
/// # Example
///
/// ```rust
/// use std::ops::{Add, Mul};
///
/// use libx::num::traits::Numeric;
///
/// // Example method using Numeric as a constraint on a sequence of elements
/// fn doubling_all<T>(sequence: &[T]) -> Vec<T>
/// where
///     T: Numeric + Copy,
///     Vec<T>: FromIterator<<T as Mul>::Output>,
/// {
///     sequence.iter().map(|&x| x * (T::ONE + T::ONE)).collect()
/// }
///
/// let observations = vec![1.5, 2.0, 3.25, 4.875, 5.5];
/// let doubled_observations = doubling_all(&observations);
/// assert_eq!(doubled_observations, vec![3.0, 4.0, 6.5, 9.75, 11.0]);
///
/// let integers = 0..8;
/// let doubled_integers: Vec<i32> = doubling_all(&integers.collect::<Vec<_>>());
/// assert_eq!(doubled_integers, vec![0, 2, 4, 6, 8, 10, 12, 14]);
/// ```
pub trait Numeric: AdditiveArithmetic + Mul + MulAssign + Copy {}

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

/// The `SignedNumeric` trait extends the functionality of the `Numeric` trait
/// to support obtaining a value's additive inverse, i.e., negation.
///
/// This trait provides the `negate` method, which mutates the value by setting it
/// to its additive inverse, and requires the implementation of the `Neg` trait for negation.
///
/// Types conforming to `SignedNumeric` must implement negation logic. By default,
/// if a type conforms to `Numeric` and `Neg`, it will automatically conform to `SignedNumeric`.
/// However, to customize negation behavior, a type can provide its own implementation of `negate`.
///
/// ## Behavior
/// The `negate` method performs the mutating operation to negate a value. If negating the value
/// results in an unrepresentable value (e.g., overflowing an integer's range), the operation
/// should either cause a trap or return an exceptional result.
///
/// # Errors
/// When negating a value leads to an unrepresentable result (e.g., overflow or underflow),
/// the operation should handle the error according to the type's design. For instance, in Rust,
/// negating `i32::MIN` will cause an overflow and potentially panic.
pub trait SignedNumeric: Numeric + Neg<Output = Self> {
    /// Negates the current value, setting it to its additive inverse.
    ///
    /// This method should mutate the value to represent its additive inverse.
    /// It is expected that negating a value that is at the edge of its type's representable
    /// range (e.g., `i32::MIN`) will either panic or return an exceptional result, depending on
    /// how the `SignedNumeric` trait is implemented for the type.
    fn negate(&mut self) {
        *self = self.neg();
    }
}

impl SignedNumeric for i8 {}

impl SignedNumeric for i16 {}

impl SignedNumeric for i32 {}

impl SignedNumeric for i64 {}

impl SignedNumeric for i128 {}

impl SignedNumeric for f32 {}

impl SignedNumeric for f64 {}

/// A trait representing binary integer types.
///
/// This trait provides a set of methods that work on binary integer types. It is designed to be
/// the foundation for all integer types, both signed and unsigned, in the standard library. Types
/// that implement this trait must also implement several other traits, such as `Hash`, `Numeric`,
/// `Rem`, `BitXor`, `BitOr`, `RemAssign`, `BitOrAssign`, `Shl`, `Shr`, `ShlAssign`, and `ShrAssign`.
///
/// The methods in this trait are typically used for working with binary representations of integers,
/// including operations like division, bit manipulation, and determining properties like sign and bit width.
///
/// # Associated Types
/// - `Self`: The type that implements this trait (e.g., `i32`, `u64`, etc.).
///
/// # Examples
/// ```
/// use libx::num::traits::BinaryInteger;
///
/// let x: i32 = 10;
/// let y: i32 = 5;
/// assert_eq!(x.quotient_and_remainder_dividing_by(y), (2, 0));
/// assert!(x.is_multiple_of(y));
/// assert_eq!(x.signum(), 1);
/// assert_eq!(x.bit_width(), 32);
/// ```
pub trait BinaryInteger:
    Hash
    + Numeric
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + BitXor
    + BitOr
    + RemAssign
    + BitOrAssign
    + Shl
    + Shr
    + ShlAssign
    + ShrAssign
{
    /// Returns the quotient and remainder when dividing this integer by `rhs`.
    ///
    /// This method calculates both the quotient and remainder for a division operation.
    /// It is equivalent to calling `div_rem` in some other languages.
    ///
    /// # Arguments
    /// - `rhs`: The divisor (right-hand side value).
    ///
    /// # Returns
    /// A tuple containing the quotient and remainder.
    ///
    /// # Examples
    /// ```
    /// use libx::num::traits::BinaryInteger;
    ///
    /// let x: i32 = 10;
    /// let y: i32 = 3;
    /// assert_eq!(x.quotient_and_remainder_dividing_by(y), (3, 1));
    /// ```
    fn quotient_and_remainder_dividing_by(self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }

    /// Returns `true` if this value is a multiple of `other`, otherwise returns `false`.
    ///
    /// This method checks if the value is evenly divisible by the given value `other`.
    ///
    /// # Arguments
    /// - `other`: The value to check for divisibility.
    ///
    /// # Returns
    /// `true` if `self` is a multiple of `other`, otherwise `false`.
    ///
    /// # Examples
    /// ```
    /// use libx::num::traits::BinaryInteger;
    ///
    /// let x: i32 = 10;
    /// let y: i32 = 5;
    /// assert!(x.is_multiple_of(y));
    ///
    /// let z: i32 = 7;
    /// assert!(!z.is_multiple_of(y));
    /// ```
    fn is_multiple_of(self, other: Self) -> bool {
        if other == Self::ZERO {
            return false;
        }

        self % other == Self::ZERO
    }

    /// Returns the sign of the integer.
    ///
    /// This method returns `-1` if the value is negative, `1` if the value is positive,
    /// and `0` if the value is zero.
    ///
    /// # Returns
    /// - `1` for positive numbers,
    /// - `-1` for negative numbers,
    /// - `0` for zero.
    ///
    /// # Examples
    /// ```
    /// use libx::num::traits::BinaryInteger;
    ///
    /// let x: i32 = 10;
    /// assert_eq!(x.signum(), 1);
    ///
    /// let y: i32 = -5;
    /// assert_eq!(y.signum(), -1);
    ///
    /// let z: i32 = 0;
    /// assert_eq!(z.signum(), 0);
    /// ```
    #[must_use]
    fn signum(self) -> Self;

    /// Returns whether this type is a signed integer type.
    ///
    /// This method returns `true` for signed integer types (e.g., `i32`, `i64`), and `false`
    /// for unsigned integer types (e.g., `u32`, `u64`).
    ///
    /// # Returns
    /// - `true` if this is a signed integer type,
    /// - `false` if this is an unsigned integer type.
    ///
    /// # Examples
    /// ```
    /// use libx::num::traits::BinaryInteger;
    ///
    /// assert!(i32::is_signed());
    /// assert!(!u32::is_signed());
    /// ```
    fn is_signed() -> bool;

    /// Returns the number of bits required to represent this integer.
    ///
    /// This method returns the number of bits in the binary representation of the integer, excluding
    /// the sign bit for signed types.
    ///
    /// # Returns
    /// The number of bits in the binary representation of the integer.
    ///
    /// # Examples
    /// ```
    /// use libx::num::traits::BinaryInteger;
    ///
    /// let x: i32 = 10;
    /// assert_eq!(x.bit_width(), 32);
    ///
    /// let y: u64 = 1000;
    /// assert_eq!(y.bit_width(), 64);
    /// ```
    fn bit_width(&self) -> usize {
        mem::size_of::<Self>() * 8
    }

    /// Returns the number of trailing zero bits in the binary representation of this integer.
    ///
    /// This method counts the number of consecutive zero bits at the rightmost part of the binary
    /// representation of the integer. This is useful for bitwise operations and optimization tasks.
    ///
    /// # Returns
    /// The number of trailing zeros in the binary representation of the integer.
    ///
    /// # Examples
    /// ```
    /// use libx::num::traits::BinaryInteger;
    ///
    /// let x: i32 = 8;  // 1000 in binary
    /// assert_eq!(x.trailing_zero_bit_count(), 3);
    ///
    /// let y: i32 = 10; // 1010 in binary
    /// assert_eq!(y.trailing_zero_bit_count(), 1);
    /// ```
    fn trailing_zero_bit_count(&self) -> usize;
}

impl BinaryInteger for u8 {
    fn signum(self) -> Self {
        Self::from(self > 0)
    }

    fn is_signed() -> bool {
        false
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u16 {
    fn signum(self) -> Self {
        Self::from(self > 0)
    }

    fn is_signed() -> bool {
        false
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u32 {
    fn signum(self) -> Self {
        Self::from(self > 0)
    }

    fn is_signed() -> bool {
        false
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u64 {
    fn signum(self) -> Self {
        Self::from(self > 0)
    }

    fn is_signed() -> bool {
        false
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for u128 {
    fn signum(self) -> Self {
        Self::from(self > 0)
    }

    fn is_signed() -> bool {
        false
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BinaryInteger for i8 {
    fn signum(self) -> Self {
        if self < 0 {
            -1
        } else {
            Self::from(self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i16 {
    fn signum(self) -> Self {
        if self < 0 {
            -1
        } else {
            Self::from(self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i32 {
    fn signum(self) -> Self {
        if self < 0 {
            -1
        } else {
            Self::from(self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i64 {
    fn signum(self) -> Self {
        if self < 0 {
            -1
        } else {
            Self::from(self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

impl BinaryInteger for i128 {
    fn signum(self) -> Self {
        if self < 0 {
            -1
        } else {
            Self::from(self > 0)
        }
    }

    fn is_signed() -> bool {
        true
    }

    fn trailing_zero_bit_count(&self) -> usize {
        self.unsigned_abs().trailing_zeros() as usize
    }
}

/// The `FixedWidthInteger` trait provides methods for binary bitwise operations,
/// bit shifts, and overflow handling for types with a fixed bit width.
///
/// It extends the `BinaryInteger` trait with additional functionality specific
/// to fixed-width nteger types, such as overflow-aware arithmetic, bit
/// manipulation, and maximum/minimum representable values.
///
/// You can use this trait to constrain or extend operations that require bitwise
/// shifts, overflow detection, or access to the type's maximum or minimum values.
pub trait FixedWidthInteger: BinaryInteger {
    /// The big-endian representation of this integer.
    ///
    /// This is the integer's value with the byte order reversed so that the most significant byte
    /// comes first in memory.
    #[must_use]
    fn big_endian(&self) -> Self;

    /// The byte-swapped representation of this integer.
    ///
    /// This method reverses the byte order of the integer's representation.
    #[must_use]
    fn byte_swapped(&self) -> Self;

    /// The number of leading zeros in this value's binary representation.
    ///
    /// This method counts the number of zeros before the first one in the binary form of the integer.
    fn leading_zero_bit_count(&self) -> usize;

    /// The little-endian representation of this integer.
    ///
    /// This is the integer's value with the byte order reversed so that the least significant byte
    /// comes first in memory.
    #[must_use]
    fn little_endian(&self) -> Self;

    /// The number of bits set to 1 in this value's binary representation.
    ///
    /// This method counts the number of ones in the binary form of the integer.
    fn nonzero_bit_count(&self) -> usize;

    /// Returns the sum of this value and the given value, along with a Boolean indicating
    /// whether overflow occurred during the operation.
    ///
    /// This method performs addition with overflow detection.
    ///
    /// # Arguments:
    /// - `other`: The value to add to `self`.
    ///
    /// # Returns:
    /// A tuple containing the result of the addition and a Boolean indicating overflow.    
    fn adding_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    /// Returns the quotient obtained by dividing this value by the given value, along with
    /// a Boolean indicating whether overflow occurred during the operation.
    ///
    /// This method performs division with overflow detection.
    ///
    /// # Arguments:
    /// - `other`: The value by which to divide `self`.
    ///
    /// # Returns:
    /// A tuple containing the result of the division and a Boolean indicating overflow.
    fn divided_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    /// Returns the product of this value and the given value, along with a Boolean indicating
    /// whether overflow occurred during the multiplication.
    ///
    /// This method performs multiplication with overflow detection.
    ///
    /// # Arguments:
    /// - `other`: The value to multiply `self` by.
    ///
    /// # Returns:
    /// A tuple containing the result of the multiplication and a Boolean indicating overflow.
    fn multiplied_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    /// Returns the remainder after dividing this value by the given value, along with a Boolean
    /// indicating whether overflow occurred during the division.
    ///
    /// This method performs division with overflow detection.
    ///
    /// # Arguments:
    /// - `other`: The value to divide `self` by.
    ///
    /// # Returns:
    /// A tuple containing the remainder of the division and a Boolean indicating overflow.
    fn remainder_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    /// Returns the difference obtained by subtracting the given value from this value, along with
    /// a Boolean indicating whether overflow occurred during the operation.
    ///
    /// This method performs subtraction with overflow detection.
    ///
    /// # Arguments:
    /// - `other`: The value to subtract from `self`.
    ///
    /// # Returns:
    /// A tuple containing the result of the subtraction and a Boolean indicating overflow.
    fn subtracting_reporting_overflow(&self, rhs: Self) -> (Self, bool);

    /// The maximum representable integer value for this type.
    ///
    /// This is the largest integer value that can be represented with the fixed width
    /// of the type.
    fn max() -> Self;

    /// The minimum representable integer value for this type.
    ///
    /// This is the smallest integer value that can be represented with the fixed width
    /// of the type.
    fn min() -> Self;
}

impl FixedWidthInteger for u8 {
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

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
    fn big_endian(&self) -> Self {
        self.to_be()
    }

    fn byte_swapped(&self) -> Self {
        self.swap_bytes()
    }

    fn leading_zero_bit_count(&self) -> usize {
        self.unsigned_abs().leading_zeros() as usize
    }

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

/// An integer type that can represent both positive and negative values.
pub trait SignedInteger: BinaryInteger + SignedNumeric {}

impl SignedInteger for i8 {}

impl SignedInteger for i16 {}

impl SignedInteger for i32 {}

impl SignedInteger for i64 {}

impl SignedInteger for i128 {}

/// An integer type that can represent only nonnegative values.
pub trait UnsignedInteger: BinaryInteger {}

impl UnsignedInteger for u8 {}

impl UnsignedInteger for u16 {}

impl UnsignedInteger for u32 {}

impl UnsignedInteger for u64 {}

impl UnsignedInteger for u128 {}

/// A trait for floating-point numeric types.
///
/// This trait provides methods for common floating-point operations such as rounding,
/// square root calculation, and comparison. It also includes methods for handling special
/// values like `NaN`, `infinity`, and `zero`, as well as inspecting and manipulating
/// the internal structure of a floating-point value (e.g., its significand, exponent, etc.).
pub trait FloatingPoint: SignedNumeric {
    /// The associated type for the exponent, which must be a signed integer type.
    ///
    /// This associated type represents the exponent of the floating-point value,
    /// and is typically a signed integer type like `i32` or `i64`.
    type Exponent: SignedInteger;

    /// Returns the smallest integer greater than or equal to `self`.
    ///
    /// This method rounds up the value to the nearest integer. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 3.5;
    /// assert_eq!(x.ceil(), 4.0);
    /// ```
    #[must_use]
    fn ceil(self) -> Self;

    /// Returns the largest integer less than or equal to `self`.
    ///
    /// This method rounds down the value to the nearest integer. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 3.5;
    /// assert_eq!(x.floor(), 3.0);
    /// ```
    #[must_use]
    fn floor(self) -> Self;

    /// Returns the fractional part of `self`.
    ///
    /// This method computes the difference between `self` and the largest integer
    /// less than or equal to `self`. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 3.5;
    /// assert_eq!(x.fract(), 0.5);
    /// ```
    #[must_use]
    fn fract(self) -> Self;

    /// Returns the integer part of `self`, truncating the fractional part.
    ///
    /// This method effectively removes the fractional part of the number.
    /// For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 3.5;
    /// assert_eq!(x.trunc(), 3.0);
    /// ```
    #[must_use]
    fn trunc(self) -> Self;

    /// Returns the exponent of the floating-point value.
    ///
    /// This method returns the exponent part of the floating-point number, which
    /// is the power of the base used for the representation of the number.
    fn exponent(self) -> Self::Exponent;

    /// Returns the floating-point classification of the value.
    ///
    /// This method categorizes the value based on its type, returning a value
    /// from `FloatingPointClassification` such as `Normal`, `Subnormal`, `NaN`, or `Infinity`.
    fn floating_point_class(&self) -> FloatingPointClassification;

    /// Returns whether the value is in canonical form.
    ///
    /// A floating-point number is in canonical form if it is represented in its
    /// standard form (without any redundant parts). This method helps in checking
    /// whether a number has been "denormalized".
    fn is_canonical(&self) -> bool;

    /// Returns whether the value is finite (i.e., not `NaN` or `Infinity`).
    ///
    /// This method returns `true` if the value is a finite number (i.e., not infinite
    /// or NaN). For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 3.0;
    /// assert_eq!(x.is_finite(), true);
    /// ```
    fn is_finite(&self) -> bool;

    /// Returns whether the value is infinite.
    ///
    /// This method returns `true` if the value is either positive or negative infinity.
    fn is_infinite(&self) -> bool;

    /// Returns whether the value is `NaN` (Not a Number).
    ///
    /// This method checks if the value is `NaN`, which represents an undefined or
    /// unrepresentable value (such as the result of dividing zero by zero).
    fn is_nan(&self) -> bool;

    /// Returns whether the value is a normal floating-point number.
    ///
    /// A "normal" floating-point number is a number that is not subnormal (denormalized)
    /// and is finite. For example, `1.0` is normal, but `1e-1000` might not be depending
    /// on the system's precision.
    fn is_normal(&self) -> bool;

    /// Returns whether the value is a signaling `NaN`.
    ///
    /// A signaling `NaN` (sNaN) is a special type of `NaN` that is used to indicate
    /// a fault or invalid operation in floating-point calculations.
    fn is_signaling_nan(&self) -> bool;

    /// Returns whether the value is a subnormal (denormalized) floating-point number.
    ///
    /// Subnormal numbers are numbers that are closer to zero than the smallest normal
    /// number. They are used to represent numbers near zero that would otherwise underflow.
    fn is_subnormal(&self) -> bool;

    /// Returns whether the value is zero.
    ///
    /// This method checks if the value is exactly zero (including `-0.0`).
    fn is_zero(&self) -> bool;

    /// Returns the greatest representable value less than `self`.
    ///
    /// This method computes the closest representable value that is smaller than `self`.
    /// It is useful for navigating values near the boundaries of the floating-point range.
    #[must_use]
    fn next_down(self) -> Self;

    /// Returns the least representable value greater than `self`.
    ///
    /// This method computes the closest representable value that is larger than `self`.
    #[must_use]
    fn next_up(self) -> Self;

    /// Returns the sign of the floating-point value.
    ///
    /// This method returns a value indicating whether the floating-point number is positive,
    /// negative, or zero.
    fn sign(&self) -> FloatingPointSign;

    /// Returns the significand (also known as the mantissa) of the floating-point value.
    ///
    /// The significand is the part of the floating-point number that represents its significant
    /// digits, without the exponent. For example, in the number `6.022e23`, the significand is `6.022`.
    #[must_use]
    fn significand(self) -> Self;

    /// Returns the unit in the last place (ULP) of the value.
    ///
    /// This method returns the smallest possible difference between `self` and another number
    /// that is greater than `self`.
    #[must_use]
    fn ulp(self) -> Self;

    /// Adds the product of `lhs` and `rhs` to `self` in place.
    ///
    /// This method performs the operation `self = self + (lhs * rhs)`, but does so without
    /// any intermediate rounding.
    fn add_product(&mut self, lhs: Self, rhs: Self);

    /// Returns the result of adding the product of `lhs` and `rhs` to `self`,
    /// without intermediate rounding.
    ///
    /// This method returns a new value equal to `self + (lhs * rhs)` but does not modify `self`.
    #[must_use]
    fn adding_product(self, lhs: Self, rhs: Self) -> Self;

    /// Replaces `self` with the remainder of `self` divided by `other`.
    ///
    /// This method computes the remainder of the division of `self` by `other`,
    /// and updates `self` to hold the result.
    fn form_remainder(&mut self, other: Self);

    /// Replaces `self` with its square root.
    ///
    /// This method calculates the square root of `self`, updating `self` in place.
    fn form_square_root(&mut self);

    /// Replaces `self` with the remainder of `self` divided by `other`, using truncating division.
    ///
    /// Truncating division discards any fractional part of the result of division.
    fn form_truncating_remainder(&mut self, other: Self);

    /// Returns whether `self` is equal to `other`.
    ///
    /// This method compares two floating-point numbers for equality. Note that `NaN` is
    /// never considered equal to any other value, including another `NaN`.
    fn is_equal_to(&self, other: Self) -> bool;

    /// Returns whether `self` is less than `other`.
    ///
    /// This method checks if `self` is less than `other`, returning `true` if so.
    fn is_less_than(&self, other: Self) -> bool;

    /// Returns whether `self` is less than or equal to `other`.
    ///
    /// This method checks if `self` is less than or equal to `other`, returning `true` if so.
    fn is_less_than_or_equal_to(&self, other: Self) -> bool;

    /// Returns whether `self` should precede or tie positions with `other` in an ascending sort.
    ///
    /// This method is useful for sorting floating-point values.
    fn is_totally_ordered_below_or_equal_to(&self, other: Self) -> bool;

    /// Returns the remainder of `self` divided by `other`.
    ///
    /// This method computes the remainder when `self` is divided by `other`, following the
    /// same behavior as the `%` operator, but without modifying the original values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 8.625;
    /// assert_eq!(x.remainder(0.75), -0.375);
    /// ```
    #[must_use]
    fn remainder(self, other: Self) -> Self;

    /// Rounds `self` to the nearest integer, modifying `self` in place.
    ///
    /// This method rounds the value of `self` to the nearest integer. The rounding follows
    /// the default rounding behavior (round half to even). For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let mut x = 2.5;
    /// x.round();
    /// assert_eq!(x, 3.0);
    /// ```
    fn round(&mut self);

    /// Rounds `self` to the nearest integer using the specified rounding rule, modifying `self`.
    ///
    /// This method rounds the value of `self` to the nearest integer using the provided
    /// `FloatingPointRoundingRule`, allowing you to control how rounding is handled (e.g.,
    /// rounding towards zero, away from zero, etc.).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::{FloatingPoint, FloatingPointRoundingRule};
    ///
    /// let mut x = 2.5;
    /// x.round_with(FloatingPointRoundingRule::Down);
    /// assert_eq!(x, 2.0);
    /// ```
    fn round_with(&mut self, rule: FloatingPointRoundingRule);

    /// Returns the result of rounding `self` to the nearest integer.
    ///
    /// This method creates a new value by rounding `self` to the nearest integer, without
    /// modifying the original value. It uses the default rounding behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 2.5;
    /// assert_eq!(x.rounded(), 3.0);
    /// ```
    #[must_use]
    fn rounded(self) -> Self;

    /// Returns the result of rounding `self` to the nearest integer using the specified
    /// rounding rule.
    ///
    /// This method creates a new value by rounding `self` to the nearest integer, using
    /// the specified `FloatingPointRoundingRule` to control the rounding behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::{FloatingPoint, FloatingPointRoundingRule};
    ///
    /// let x = 2.5;
    /// assert_eq!(x.rounded_with(FloatingPointRoundingRule::Down), 2.0);
    /// ```
    #[must_use]
    fn rounded_with(self, rule: FloatingPointRoundingRule) -> Self;

    /// Returns the square root of `self`.
    ///
    /// This method computes the square root of `self` and returns the result. If `self` is
    /// negative, the result will be `NaN`. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 4.0;
    /// assert_eq!(x.square_root(), 2.0);
    /// ```
    #[must_use]
    fn square_root(self) -> Self;

    /// Returns the remainder of `self` divided by `other`, using truncating division.
    ///
    /// This method computes the remainder of the division of `self` by `other`, using truncating
    /// division (i.e., the remainder is computed as though the result was rounded toward zero).
    /// For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// let x = 5.5;
    /// let y = 2.0;
    /// assert_eq!(x.truncating_remainder(y), 1.5);
    /// ```
    #[must_use]
    fn truncating_remainder(self, other: Self) -> Self;

    /// Returns the greatest finite representable value.
    ///
    /// This method returns the largest finite number that can be represented by the floating-point
    /// type. For example, for `f32`, it will return `f32::MAX`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::greatest_finite_magnitude(), f32::MAX);
    /// ```
    fn greatest_finite_magnitude() -> Self;

    /// Returns positive infinity.
    ///
    /// This method returns the positive infinity value for the floating-point type. It represents
    /// values that exceed the maximum finite value. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::infinity(), f32::INFINITY);
    /// ```
    fn infinity() -> Self;

    /// Returns the least positive number that is representable.
    ///
    /// This method returns the smallest non-zero positive number that can be represented by
    /// the floating-point type. For example, `f32::MIN_POSITIVE` is the smallest positive number
    /// representable by `f32`.
    fn least_nonzero_magnitude() -> Self;

    /// Returns the least positive normal number.
    ///
    /// This method returns the smallest positive normal number that can be represented, which
    /// is distinct from subnormal (denormalized) numbers. For example, `f32::LEAST_NORMAL` is
    /// the smallest normal number for `f32`.
    fn least_normal_magnitude() -> Self;

    /// Returns `NaN` (Not a Number).
    ///
    /// This method returns a quiet `NaN` value for the floating-point type. `NaN` represents
    /// undefined or unrepresentable values, such as the result of `0.0 / 0.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert!(f32::nan().is_nan());
    /// ```
    fn nan() -> Self;

    /// Returns the mathematical constant π (pi).
    ///
    /// This method returns the constant `π`, which is approximately equal to `3.14159`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::pi(), 3.1415927);
    /// ```
    fn pi() -> Self;

    /// Returns the radix (base) used for exponentiation.
    ///
    /// This method returns the base used for representing floating-point numbers in the given
    /// type, usually 2 for binary floating-point types. For example, `f32::radix()` returns 2.
    fn radix() -> Self;

    /// Returns a signaling NaN (Not a Number).
    ///
    /// This method returns a signaling `NaN`, which is a special `NaN` value that can be used to
    /// indicate an invalid operation that should trigger an exception.
    fn signaling_nan() -> Self;

    /// Returns the unit in the last place (ULP) of one.
    ///
    /// This method returns the smallest possible difference between `1.0` and the next larger
    /// representable value. This is often used to measure the precision of floating-point numbers.
    fn ulp_of_one() -> Self;

    /// Returns the greater of two values.
    ///
    /// This method returns the larger of `x` and `y`. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::maximum(3.0, 4.0), 4.0);
    /// ```
    fn maximum(x: Self, y: Self) -> Self;

    /// Returns the value with the greater magnitude.
    ///
    /// This method returns the value with the greater absolute value (ignoring the sign). For
    /// example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::maximum_magnitude(-3.0, 2.0), -3.0);
    /// ```
    fn maximum_magnitude(x: Self, y: Self) -> Self;

    /// Returns the lesser of two values.
    ///
    /// This method returns the smaller of `x` and `y`. For example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::minimum(3.0, 4.0), 3.0);
    /// ```
    fn minimum(x: Self, y: Self) -> Self;

    /// Returns the value with the lesser magnitude.
    ///
    /// This method returns the value with the smaller absolute value (ignoring the sign). For
    /// example:
    ///
    /// ```rust
    /// use libx::num::traits::FloatingPoint;
    ///
    /// assert_eq!(f32::minimum_magnitude(3.0, -2.0), -2.0);
    /// ```
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
        let bits = self.to_bits();

        if self.is_nan() || self.is_infinite() {
            return self;
        }

        let mut next_bits = bits;

        if self == 0.0 {
            next_bits = 1;
        } else if self > 0.0 {
            next_bits += 1;
        } else {
            next_bits = bits.wrapping_add(1);
        }

        let next_value = Self::from_bits(next_bits);

        (next_value - self).abs()
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
        self - (self / other).rounded() * other
    }

    fn round(&mut self) {
        *self = self.rounded();
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
        let int_part = self.trunc(); // Get the integer part (floor for positive, ceiling for negative)
        let frac_part = self.fract(); // Calculate the fractional part

        // Check if the fractional part is exactly 0.5 or -0.5 (this is halfway)
        if frac_part == 0.5 || frac_part == -0.5 {
            if self > 0.0 {
                return int_part + 1.0; // Round away from zero for positive values
            }

            return int_part - 1.0; // Round away from zero for negative values
        }

        // In all other cases, round to the nearest integer
        if frac_part >= 0.5 {
            int_part + 1.0 // Round up
        } else {
            int_part // Round down
        }
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
        if x.abs() > y.abs() {
            x
        } else {
            y
        }
    }

    fn minimum(x: Self, y: Self) -> Self {
        x.min(y)
    }

    fn minimum_magnitude(x: Self, y: Self) -> Self {
        if x.abs() < y.abs() {
            x
        } else {
            y
        }
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
        self.fract()
    }

    fn ulp(self) -> Self {
        let bits = self.to_bits();

        if self.is_nan() || self.is_infinite() {
            return self;
        }

        let mut next_bits = bits;

        if self == 0.0 {
            next_bits = 1;
        } else if self > 0.0 {
            next_bits += 1;
        } else {
            next_bits = bits.wrapping_add(1);
        }

        let next_value = Self::from_bits(next_bits);

        (next_value - self).abs()
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
        self - (self / other).rounded() * other
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
        let int_part = self.trunc();
        let frac_part = self - int_part;

        if frac_part == 0.5 || frac_part == -0.5 {
            if self > 0.0 {
                return int_part + 1.0;
            }
            return int_part - 1.0;
        }

        // In all other cases, round to the nearest integer
        if frac_part >= 0.5 {
            int_part + 1.0 // Round up
        } else {
            int_part // Round down
        }
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
        if x.abs() > y.abs() {
            x
        } else {
            y
        }
    }

    fn minimum(x: Self, y: Self) -> Self {
        x.min(y)
    }

    fn minimum_magnitude(x: Self, y: Self) -> Self {
        if x.abs() < y.abs() {
            x
        } else {
            y
        }
    }
}

/// Represents the classification of a floating-point value, based on its sign and magnitude.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatingPointClassification {
    /// A value equal to negative infinity.
    NegativeInfinity,

    /// A negative value that uses the full precision of the floating-point type.
    NegativeNormal,

    /// A negative, nonzero number that does not use the full precision of the floating-point type.
    NegativeSubnormal,

    /// A value equal to zero with a negative sign.
    NegativeZero,

    /// A value equal to positive infinity.
    PositiveInfinity,

    /// A positive value that uses the full precision of the floating-point type.
    PositiveNormal,

    /// A positive, nonzero number that does not use the full precision of the floating-point type.
    PositiveSubnormal,

    /// A value equal to zero with a positive sign.
    PositiveZero,

    /// A silent NaN (“Not a Number”) value, which does not signal any exceptions.
    QuietNaN,

    /// A signaling NaN (“Not a Number”) value, which is intended to signal exceptions when used.
    SignalingNaN,
}

/// Represents the sign of a floating-point value.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatingPointSign {
    /// The sign for a negative floating-point value.
    Minus,

    /// The sign for a positive floating-point value.
    Plus,
}

/// Defines different rounding rules used in floating-point operations.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatingPointRoundingRule {
    /// Round to the closest allowed value whose magnitude is greater than or equal to that of the source.
    AwayFromZero,

    /// Round to the closest allowed value that is less than or equal to the source.
    Down,

    /// Round to the closest allowed value; if two values are equally close, the one with greater magnitude is chosen.
    ToNearestOrAwayFromZero,

    /// Round to the closest allowed value; if two values are equally close, the even one is chosen (bankers' rounding).
    ToNearestOrEven,

    /// Round to the closest allowed value whose magnitude is less than or equal to that of the source.
    TowardZero,

    /// Round to the closest allowed value that is greater than or equal to the source.
    Up,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_multiplication() {
        // Multiplication and multiplication assignment
        let a = 3;
        let b = 4;

        assert_eq!(a * b, 12); // a * b

        let mut c = a;
        c *= b; // a *= b
        assert_eq!(c, 12);

        // Testing with the multiplicative identity (ONE)
        assert_eq!(a * i8::ONE, a);
        assert_eq!(b * i8::ONE, b);
    }

    #[test]
    fn test_numeric_operations_with_floats() {
        // Test with floating-point numbers
        let a = 3.5;
        let b = 2.0;

        assert_eq!(a * b, 7.0);
        assert_eq!(a + b, 5.5);
        assert_eq!(a - b, 1.5);
    }

    #[test]
    fn test_numeric_operations_with_negatives() {
        // Test numeric operations on signed integers
        let a = -5;
        let b = -10;

        assert_eq!(a + b, -15);
        assert_eq!(a - b, 5);
        assert_eq!(a * b, 50);
    }

    #[test]
    fn test_signed_numeric_negate() {
        // Testing negation on different types
        let mut value = 10;
        value.negate();
        assert_eq!(value, -10);

        value = -20;
        value.negate();
        assert_eq!(value, 20);

        let mut value = 3.5;
        value.negate();
        assert_eq!(value, -3.5);

        value = -4.75;
        value.negate();
        assert_eq!(value, 4.75);
    }

    #[test]
    #[should_panic = "attempt to negate with overflow"]
    fn test_signed_numeric_overflow() {
        let mut first = i8::MIN;
        first.negate();

        let mut second = i16::MIN;
        second.negate();

        let mut third = i32::MIN;
        third.negate();

        let mut fourth = i64::MIN;
        fourth.negate();

        let mut fifth = i128::MIN;
        fifth.negate();
    }

    // Test ULP of a positive f32
    #[test]
    fn test_ulp_of_positive_float() {
        let value: f32 = 1.0;
        let ulp_value = value.ulp();
        assert_eq!(
            ulp_value,
            f32::EPSILON,
            "The ULP of 1.0 should be the same as f32::EPSILON"
        );
    }

    // Test ULP of a negative f32
    #[test]
    fn test_ulp_of_negative_float() {
        let value: f32 = -1.0;
        let ulp_value = value.ulp();
        assert_eq!(
            ulp_value,
            f32::ulp_of_one(),
            "The ULP of -1.0 should be the same as f32::EPSILON"
        );
    }

    // Test ULP of a very small f32 close to zero
    #[test]
    fn test_ulp_of_small_float() {
        let value: f32 = 1.0e-10;
        let ulp_value = value.ulp();
        assert!(
            ulp_value > 0.0,
            "The ULP of a small number should be greater than zero"
        );
    }

    // Test ULP of a very large f32
    #[test]
    fn test_ulp_of_large_float() {
        let value: f32 = 1.0e10;
        let ulp_value = value.ulp();
        assert!(
            ulp_value > 0.0,
            "The ULP of a large number should be greater than zero"
        );
    }

    // Test ULP of NaN
    #[test]
    fn test_ulp_of_nan() {
        let value: f32 = f32::NAN;
        let ulp_value = value.ulp();
        assert!(ulp_value.is_nan(), "The ULP of NaN should be NaN");
    }

    // Test ULP of Infinity
    #[test]
    fn test_ulp_of_infinity() {
        let value: f32 = f32::INFINITY;
        let ulp_value = value.ulp();
        assert!(
            ulp_value > 0.0,
            "The ULP of a very large number should be greater than zero"
        );
    }

    // Test ULP of a double precision float
    #[test]
    fn test_ulp_of_double() {
        let value: f64 = 1.0;
        let ulp_value = value.ulp();
        assert_eq!(
            ulp_value,
            f64::ulp_of_one(),
            "The ULP of 1.0 should be the same as f64::EPSILON"
        );
    }

    // Test ULP of a very small f64
    #[test]
    fn test_ulp_of_small_double() {
        let value: f64 = 1.0e-10;
        let ulp_value = value.ulp();
        assert!(
            ulp_value > 0.0,
            "The ULP of a small number should be greater than zero"
        );
    }
}
