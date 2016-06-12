
use std::marker;
use std::ops::{Add, Sub};

// TODO: mm: Implement from_str_radix()

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct U12(u16);

// MARK: - Public Constants

pub const MAX: U12 = U12(0xFFF);
pub const MIN: U12 = U12(0x000);

// MARK: - Implementation

impl U12 {

  /// Returns the smallest value that can be represented by this integer type.
  pub fn min_value() -> Self {
    MIN
  }

  /// Returns the largest value that can be represented by this integer type.
  pub fn max_value() -> Self {
    MAX
  }

  /// Returns the number of ones in the binary representation of `self`.
  pub fn count_ones(self) -> u32 {
    self.0.count_ones()
  }

  /// Returns the number of zeros in the binary representation of `self`.
  pub fn count_zeros(self) -> u32 {
    self.0.count_zeros() - 4
  }

  /// Returns the number of leading zeros in the binary representation of `self`.
  pub fn leading_zeros(self) -> u32 {
    self.0.leading_zeros() - 4
  }

  /// Returns the number of trailing zeros in the binary representation of `self`.
  pub fn trailing_zeros(self) -> u32 {
    self.0.trailing_zeros()
  }

  /// Checked integer addition. 
  /// Computes `self + other`, returning `None` if overflow occurred.
  ///
  /// # Examples
  /// Basic usage:
  /// 
  /// ```
  /// use twelve_bit::u12::U12;
  ///
  /// assert_eq!(U12::from(1u8).checked_add(1u8.into()), Some(U12::from(2u8)));
  /// assert_eq!(U12::max_value().checked_add(1u8.into()), None);
  /// ```
  pub fn checked_add(self, other: Self) -> Option<Self> {
    match self.0 + other.0 {
      result @ 0...4095 => Some(U12(result)),
      _ => None
    }
  }

  /// Saturating integer addition. 
  /// Computes `self + other`, saturating at the numeric bounds instead of overflowing.
  ///
  /// # Examples
  /// Basic usage:
  /// 
  /// ```
  /// use twelve_bit::u12::U12;
  ///
  /// assert_eq!(U12::from(1u8).saturating_add(1u8.into()), U12::from(2u8));
  /// assert_eq!(U12::max_value().saturating_add(1u8.into()), U12::max_value());
  /// ```
  pub fn saturating_add(self, other: Self) -> Self {
    match self.0 + other.0 {
      result @ 0...4095 => U12(result),
      _ => Self::max_value()
    }
  }

  /// Wrapping (modular) addition. 
  /// Computes `self + other`, wrapping around at the boundary of the type.
  ///
  /// # Examples
  /// Basic usage:
  /// 
  /// ```
  /// use twelve_bit::u12::U12;
  ///
  /// assert_eq!(U12::from(1u8).wrapping_add(1u8.into()), U12::from(2u8));
  /// assert_eq!(U12::max_value().wrapping_add(3u8.into()), U12::from(2u8));
  /// ```
  pub fn wrapping_add(self, other: Self) -> Self {
    U12((self.0 + other.0) & 0xFFF)
  }

  /// Checked integer subtraction. 
  /// Computes `self - other`, returning `None` if underflow occurred.
  ///
  /// # Examples
  /// Basic usage:
  /// 
  /// ```
  /// use twelve_bit::u12::U12;
  ///
  /// assert_eq!(U12::from(1u8).checked_sub(1u8.into()), Some(U12::from(0u8)));
  /// assert_eq!(U12::min_value().checked_sub(1u8.into()), None);
  /// ```
  pub fn checked_sub(self, other: Self) -> Option<Self> {
    match self.0.checked_sub(other.0) {
      Some(value) => Some(U12(value)),
      None => None
    }
  }

  /// Saturating integer subtraction. 
  /// Computes `self - other`, saturating at the numeric bounds instead of overflowing.
  ///
  /// # Examples
  /// Basic usage:
  /// 
  /// ```
  /// use twelve_bit::u12::U12;
  ///
  /// assert_eq!(U12::from(1u8).saturating_sub(1u8.into()), U12::min_value());
  /// assert_eq!(U12::min_value().saturating_sub(5u8.into()), U12::min_value());
  /// ```
  pub fn saturating_sub(self, other: Self) -> Self {
    U12(self.0.saturating_sub(other.0))
  }
  
  /// Wrapping (modular) subtraction. 
  /// Computes `self - other`, wrapping around at the boundary of the type.
  ///
  /// # Examples
  /// Basic usage:
  /// 
  /// ```
  /// use twelve_bit::u12::*;
  ///
  /// assert_eq!(U12::from(1u8).wrapping_sub(1u8.into()), U12::min_value());
  /// assert_eq!(U12::min_value().wrapping_sub(5u8.into()), (0xFFB as u16).unchecked_into());
  /// ```
  pub fn wrapping_sub(self, other: Self) -> Self {
    U12(self.0.wrapping_sub(other.0) & 0xFFF)
  }

}

// MARK: - Non-Failable Conversions - From Smaller Types

impl From<u8> for U12 {
  fn from(small: u8) -> Self {
    U12(small as u16)
  }
}

// MARK: - Non-Failable Conversions - Into Larger Types

/// Implements From<U12> for the specified type.
macro_rules! impl_from_u12 {
  ($result:path) => {
    impl From<U12> for $result {
      fn from(small: U12) -> Self {
        small.0 as Self
      }
    }
  }
}

impl_from_u12!(u16);
impl_from_u12!(u32);
impl_from_u12!(u64);
impl_from_u12!(usize);

// MARK: - Failable Conversions - From Larger Types

/// Trait for implementing failable conversions in a generic way.
pub trait FailableInto<T> where Self: marker::Sized, T: marker::Sized {

  /// Returns the receiver as `Some(T)` if non-truncating, or `None`.
  fn failable_into(self) -> Option<T>;

  /// Returns the receiver as `T` by using `convert_as()` and unwrapping the result.
  ///
  /// # Panics
  /// This method will panic if `convert_as` fails.
  fn unchecked_into(self) -> T {
    self.failable_into().unwrap()
  }

}

/// Implements FailableAs<U12> for the specified type.
macro_rules! impl_failable_into_u12 {
  ($from:path) => {
    impl FailableInto<U12> for $from {
      fn failable_into(self) -> Option<U12> {
        if self > 0xFFF {
          None
        } else {
          Some(U12(self as u16))
        }
      }
    }
  }
}

impl_failable_into_u12!(u16);
impl_failable_into_u12!(u32);
impl_failable_into_u12!(u64);
impl_failable_into_u12!(usize);

// MARK: - Default Value

impl Default for U12 {
  fn default() -> Self {
    U12::min_value()
  }
}

// MARK: - Add

impl Add<U12> for U12 {
  type Output = U12;
  fn add(self, other: U12) -> Self::Output {
    match self.checked_add(other) {
      Some(result) => result,
      None => panic!("arithmetic overflow")
    }
  }
}

impl<'a> Add<U12> for &'a U12 {
  type Output = U12;
  fn add(self, other: U12) -> Self::Output {
    (*self).add(other)
  }
}

impl<'a> Add<&'a U12> for U12 {
  type Output = U12;
  fn add(self, other: &'a U12) -> Self::Output {
    self.add(*other)
  }
}

impl<'a,'b> Add<&'a U12> for &'b U12 {
  type Output = U12;
  fn add(self, other: &'a U12) -> Self::Output {
    (*self).add(*other)
  }
}

// MARK: - Sub

impl Sub<U12> for U12 {
  type Output = U12;
  fn sub(self, other: U12) -> Self::Output {
    match self.checked_sub(other) {
      Some(result) => result,
      None => panic!("arithmetic underflow")
    }
  }
}

impl<'a> Sub<U12> for &'a U12 {
  type Output = U12;
  fn sub(self, other: U12) -> Self::Output {
    (*self).sub(other)
  }
}

impl<'a> Sub<&'a U12> for U12 {
  type Output = U12;
  fn sub(self, other: &'a U12) -> Self::Output {
    self.sub(*other)
  }
}

impl<'a,'b> Sub<&'a U12> for &'b U12 {
  type Output = U12;
  fn sub(self, other: &'a U12) -> Self::Output {
    (*self).sub(*other)
  }
}
