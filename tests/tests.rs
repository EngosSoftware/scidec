//! # Smoke tests
//!
//! Functions exposed by this library are intensively tested using unit tests.
//! Smoke tests check only the correctness of the library interface.

use scidec::{bid128_from_string, number_from_string, Number};

#[test]
fn test_number_from_string() {
  assert!((Number::Finite(false, 0, 3, -7) == number_from_string("0.00003E-02")));
}

#[test]
fn test_bid128_from_string() {
  let (actual, status) = bid128_from_string("0.00003E-02");
  assert_eq!(0x3032000000000000, actual.w[1]);
  assert_eq!(0x0000000000000003, actual.w[0]);
  assert_eq!(0x0, status);
}
