//! Utility functions for unit tests.

use crate::bid128::bid128_from_string_rnd;
use crate::recognizer::Rounding;

const BID128_INPUT: &str = include_str!("test_cases.in");

#[test]
fn test_input_cases() {
  for (i, mut line) in BID128_INPUT.lines().enumerate() {
    line = line.trim();
    if !line.is_empty() && !line.starts_with('#') {
      let mut columns = line.split(' ');
      let rounding = columns.next().unwrap().parse::<i32>().unwrap();
      let input = columns.next().unwrap().trim_matches('"').replace('_', " ");
      let mut bid = columns.next().unwrap().trim_matches('[').trim_matches(']').split(',');
      let expected_w1 = u64::from_str_radix(bid.next().unwrap(), 16).unwrap();
      let expected_w0 = u64::from_str_radix(bid.next().unwrap(), 16).unwrap();
      let expected_status = u32::from_str_radix(columns.next().unwrap(), 16).unwrap();
      let (actual, actual_status) = bid128_from_string_rnd(&input, rounding.into());
      let actual_w1 = actual.w[1];
      let actual_w0 = actual.w[0];
      let line_no = i + 1;
      assert_eq!(
        expected_w1, actual_w1,
        "[{}] w1:\nexpected: {expected_w1:016x}\n  actual: {actual_w1:016x}\n",
        line_no
      );
      assert_eq!(
        expected_w0, actual_w0,
        "[{}] w0:\nexpected: {expected_w0:016x}\n  actual: {actual_w0:016x}\n",
        line_no
      );
      assert_eq!(
        expected_status, actual_status,
        "[{}] status:\nexpected: {expected_status:02x}\n  actual: {actual_status:02x}\n",
        line_no
      );
    }
  }
}

#[test]
fn test_check() {
  let s = "na";
  let (actual, actual_status) = bid128_from_string_rnd(s, Rounding::ToNearest);
  assert_eq!(0x7c00000000000000, actual.w[1], "{:016x}", actual.w[1]);
  assert_eq!(0x0000000000000000, actual.w[0], "{:016x}", actual.w[0]);
  assert_eq!(0x00, actual_status);
}
