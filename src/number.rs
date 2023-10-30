//! # Number parser

use crate::recognizer::{recognize, Value};
use crate::Rounding;

/// Parsed number.
#[derive(Eq, PartialEq)]
pub enum Number {
  /// Variant representing a finite number.
  Finite(
    /// Flag indicating if the number is signed,
    /// `true` signed (`-` minus), `false` unsigned (`+` plus).
    bool,
    /// Higher 64-bits of the number.
    u64,
    /// Lower 64-bits of the number.
    u64,
    /// Exponent.
    i32,
  ),
  /// Variant representing an infinity.
  Infinite(
    /// Flag indicating if the infinity is signed,
    /// `true` positive infinity, `false` negative infinity.
    bool,
  ),
  /// Variant representing an invalid number.
  NaN(
    /// Flag indicating if the value is signed,
    /// `true` positive infinity, `false` negative infinity.
    bool,
    /// Flag indicating if this is a signalling NaN,
    /// `true` signaling, `false` quiet.
    bool,
  ),
}

/// Parses a number properties from text in scientific notation.
///
/// # Examples
///
/// Input text represents a finite number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("1234.5678e-2");
/// match result {
///   Number::Finite(false, 0, 12345678, -6) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents a positive infinity.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("inf");
/// match result {
///   Number::Infinite(false) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents a negative infinity.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("-Infinity");
/// match result {
///   Number::Infinite(true) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents quiet not-a-number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("NaN");
/// match result {
///   Number::NaN(false, false) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents signaling not-a-number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("SNaN");
/// match result {
///   Number::NaN(false, true) => {}
///   _ => panic!()
/// }
/// ```
pub fn number_from_string(input: &str) -> Number {
  match recognize(input, 34, Rounding::ToNearest) {
    Value::Finite(sign, value, exponent, _status) => Number::Finite(sign, (value >> 64) as u64, value as u64, exponent),
    Value::Infinity(sign) => Number::Infinite(sign),
    Value::NaN(sign, signaling) => Number::NaN(sign, signaling),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eq() {
    assert!((Number::Finite(false, 0, 0, 0) == Number::Finite(false, 0, 0, 0)));
    assert!((Number::Finite(false, 0, 0, 0) != Number::Infinite(false)));
    assert!((Number::Infinite(true) != Number::Infinite(false)));
    assert!((Number::Infinite(true) == Number::Infinite(true)));
    assert!((Number::NaN(true, true) != Number::NaN(false, false)));
    assert!((Number::NaN(false, false) == Number::NaN(false, false)));
    Number::Infinite(false).assert_receiver_is_total_eq();
  }
}
